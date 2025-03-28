use anyhow::{anyhow, Result};
use clap::{ArgAction, Parser};
use extractor::{get_extractor, register_extractors};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

mod extractor;
mod trust;

/// Extract compressed archives automatically
#[derive(Parser, Debug)]
#[clap(
    name = "extr",
    before_help = r#"

  ███████╗██╗  ██╗████████╗██████╗ 
  ██╔════╝╚██╗██╔╝╚══██╔══╝██╔══██╗
  █████╗   ╚███╔╝    ██║   ██████╔╝
  ██╔══╝   ██╔██╗    ██║   ██╔══██╗
  ███████╗██╔╝ ██╗   ██║   ██║  ██║
  ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
     Effortless File Extraction
    "#,
    after_help = r#"
Examples:
  extr file.zip                Extract to current directory
  extr *.tar.gz -o ~/unpacked  Extract multiple archives
  extr --health                Verify system compatibility

📦 Supports 30+ archive formats | 🚀 No flags, no hazzle
    "#,
    about = "Forget the tar flags tango - just type the filename and extract!",
    version,
    author = "Fredrik Söderström <tirithen@gmail.com>",
    long_about = "The archive extractor that makes unpacking files as easy as typing their name. No more memorizing complex flags or commands - just point extr at any archive and let the magic happen. Supporting 30+ formats, extr is the Swiss Army knife for all your file extraction needs."
)]
struct Args {
    /// Archive files to extract
    #[clap(name = "FILE")]
    files: Vec<PathBuf>,

    /// Check system health for supported archive formats
    #[clap(long, action = ArgAction::SetTrue)]
    health: bool,

    /// Extract to a specific directory
    #[clap(short, long, value_name = "DIR")]
    output_dir: Option<PathBuf>,

    /// Verbose output
    #[clap(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

fn main() -> Result<()> {
    register_extractors();

    let args = Args::parse();

    if args.health {
        return print_health_check();
    }

    if args.files.is_empty() {
        return Err(anyhow!(
            "💥 Whoops! At least one FILE is required. Run $ extr --help for details."
        ));
    }

    let extractors = args
        .files
        .iter()
        .map(|file| match get_extractor(file) {
            Some(extractor) => match extractor.get_verified_binary() {
                Ok(_) => Ok((file, extractor)),
                Err(error) => Err(error),
            },
            None => Err(anyhow!(
                "💥 Whoops! Unable to find a supported extractor for {}, is the file an archive?",
                file.to_string_lossy()
            )),
        })
        .collect::<Vec<_>>();

    for result in &extractors {
        if let Err(error) = result {
            eprintln!("{error}");
            exit(1);
        }
    }

    let output_dir = args.output_dir.unwrap_or(PathBuf::from_str(".").unwrap());
    for result in extractors {
        let (file, extractor) = result?;
        extractor.extract(file, &output_dir, args.verbose)?;
    }

    Ok(())
}

fn print_health_check() -> Result<()> {
    use extractor::get_health_info;
    use owo_colors::OwoColorize;

    let health_info = get_health_info();

    println!("{}", "Archive format support health check\n".bold());
    println!(
        "To support a format at least one of the compatible\nbinaries must be installed on the system.\n"
    );
    println!(
        "{:<15} {:<30}",
        "Format".bold(),
        "Available Binaries".bold()
    );
    println!("{}", "―".repeat(50));

    for (format, binaries) in health_info {
        let mut available = Vec::new();
        let mut missing = Vec::new();

        for (bin, installed) in binaries {
            if installed {
                available.push(format!("{} {}", "✓".green(), bin));
            } else {
                missing.push(format!("{} {}", "✘".red(), bin));
            }
        }

        let all_bins = [available, missing].concat().join(", ");
        println!("{:<15} {}", format!(".{}", format).blue(), all_bins);
    }

    println!("\n{}: installed, {}: missing", "✓".green(), "✘".red());
    Ok(())
}
