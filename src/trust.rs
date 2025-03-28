use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use crossbeam_channel::{select, tick, unbounded};

pub fn is_trusted_bin_path(path: &Path) -> bool {
    let canonical_path = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(_) => return false,
    };

    #[cfg(target_os = "linux")]
    {
        let valid_prefixes = [
            Path::new("/bin"),
            Path::new("/sbin"),
            Path::new("/usr/bin"),
            Path::new("/usr/sbin"),
            Path::new("/usr/local/bin"),
            Path::new("/usr/local/sbin"),
        ];

        valid_prefixes
            .iter()
            .any(|prefix| canonical_path.starts_with(prefix))
    }

    #[cfg(target_os = "macos")]
    {
        let valid_prefixes = [
            Path::new("/usr/bin"),
            Path::new("/usr/sbin"),
            Path::new("/bin"),
            Path::new("/sbin"),
            Path::new("/usr/local/bin"),
            Path::new("/usr/local/sbin"),
            Path::new("/opt/homebrew/bin"),
            Path::new("/opt/homebrew/sbin"),
            Path::new("/opt/local/bin"),
            Path::new("/opt/local/sbin"),
        ];

        valid_prefixes
            .iter()
            .any(|prefix| canonical_path.starts_with(prefix))
    }

    #[cfg(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    {
        let valid_prefixes = [
            Path::new("/bin"),
            Path::new("/sbin"),
            Path::new("/usr/bin"),
            Path::new("/usr/sbin"),
            Path::new("/usr/local/bin"),
            Path::new("/usr/local/sbin"),
        ];

        valid_prefixes
            .iter()
            .any(|prefix| canonical_path.starts_with(prefix))
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    {
        false
    }
}

pub fn run_command(mut cmd: Command, verbose: bool) -> Result<()> {
    cmd.stdin(Stdio::piped());
    cmd.stdout(if verbose {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    let (sender, receiver) = unbounded();
    let timer = tick(std::time::Duration::from_millis(100));

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdin = child.stdin.take();

    if let Some(mut stdout) = stdout {
        let sender = sender.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match stdout.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let _ = io::stdout().write_all(&buf[..n]);
                        let _ = io::stdout().flush();
                    }
                    Err(_) => break,
                }
            }
            let _ = sender.send(());
        });
    }

    if let Some(mut stderr) = stderr {
        let sender = sender.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match stderr.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let _ = io::stderr().write_all(&buf[..n]);
                        let _ = io::stderr().flush();
                    }
                    Err(_) => break,
                }
            }
            let _ = sender.send(());
        });
    }

    if let Some(mut stdin) = stdin {
        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match io::stdin().read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let _ = stdin.write_all(&buf[..n]);
                        let _ = stdin.flush();
                    }
                    Err(_) => break,
                }
            }
        });
    }

    let status = loop {
        select! {
            recv(receiver) -> _ => {},
            recv(timer) -> _ => {
                if let Some(status) = child.try_wait()? {
                    break status;
                }
            }
        }
    };

    if !status.success() {
        return Err(anyhow!(
            "💥 Whoops! Command failed with exit status {status}"
        ));
    }

    Ok(())
}
