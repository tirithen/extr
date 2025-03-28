use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct LhaExtractor;

impl Extractor for LhaExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["lzh", "lha"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["lha", "lhasa", "unar", "7z"]
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let mut cmd = Command::new(binary);

        match binary.file_name().unwrap().to_str().unwrap() {
            "lha" | "lhasa" => {
                cmd.arg("x");
                if verbose {
                    cmd.arg("-v");
                }
                cmd.arg("-f").arg(file).arg(output_dir);
            }
            "unar" => {
                cmd.arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-q");
                }
            }
            "7z" => {
                cmd.arg("x")
                    .arg("-y")
                    .arg(format!("-o{}", output_dir.display()))
                    .arg(file);
                if !verbose {
                    cmd.arg("-bso0").arg("-bd");
                }
            }
            _ => anyhow::bail!("Unsupported LHA/LZH extraction tool: {}", binary.display()),
        }

        Ok(cmd)
    }
}
