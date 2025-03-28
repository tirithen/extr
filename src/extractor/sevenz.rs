use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct SevenzExtractor;

impl Extractor for SevenzExtractor {
    fn file_extensions(&self) -> Vec<&str> {
        vec!["7z"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        #[cfg(target_os = "macos")]
        {
            vec!["unar", "7z", "7za", "7zr", "bsdtar"]
        }
        #[cfg(not(target_os = "macos"))]
        {
            vec!["7z", "7za", "7zr", "bsdtar"]
        }
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let binary_name = binary
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid binary path"))?;

        let mut cmd = Command::new(binary);

        match binary_name {
            "7z" | "7za" | "7zr" => {
                cmd.arg("x")
                    .arg(format!("-o{}", output_dir.display()))
                    .arg(file)
                    .arg("-y");

                if !verbose {
                    cmd.arg("-bso0").arg("-bd");
                }
            }
            "bsdtar" => {
                cmd.arg("-xf").arg(file).arg("-C").arg(output_dir);

                if verbose {
                    cmd.arg("-v");
                }
            }
            "unar" => {
                cmd.arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-q");
                }
            }
            _ => anyhow::bail!("Unsupported 7z extraction tool: {}", binary_name),
        }

        Ok(cmd)
    }
}
