use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct ZipExtractor;

impl Extractor for ZipExtractor {
    fn file_extensions(&self) -> Vec<&str> {
        vec!["zip", "jar", "war"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        #[cfg(target_os = "macos")]
        {
            vec!["unar", "unzip", "7z", "bsdtar", "jar"]
        }
        #[cfg(not(target_os = "macos"))]
        {
            vec!["unzip", "7z", "bsdtar", "jar"]
        }
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let binary_name = binary.file_name().unwrap().to_str().unwrap();
        let mut cmd = Command::new(binary);

        match binary_name {
            "unar" => {
                cmd.arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-q");
                }
            }
            "unzip" => {
                if !verbose {
                    cmd.arg("-q");
                }
                cmd.arg(file).arg("-d").arg(output_dir);
            }
            "7z" => {
                cmd.arg("x").arg("-y").arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-bd");
                }
            }
            "bsdtar" => {
                cmd.arg("-xf").arg(file).arg("-C").arg(output_dir);
                if verbose {
                    cmd.arg("-v");
                }
            }
            "jar" => {
                cmd.arg("xf").arg(file).current_dir(output_dir);
                if verbose {
                    cmd.arg("-v");
                }
            }
            _ => anyhow::bail!("Unsupported zip extraction tool: {}", binary_name),
        }

        Ok(cmd)
    }
}
