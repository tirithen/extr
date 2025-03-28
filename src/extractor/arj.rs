use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct ArjExtractor;

impl Extractor for ArjExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["arj"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        #[cfg(target_os = "macos")]
        {
            vec!["unar", "arj"]
        }
        #[cfg(not(target_os = "macos"))]
        {
            vec!["arj", "7z"]
        }
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let mut cmd = Command::new(binary);
        let bin_name = binary.file_name().unwrap().to_str().unwrap();

        match bin_name {
            "unar" => {
                cmd.arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-q");
                }
            }
            "arj" => {
                cmd.arg("x").arg("-y").arg(file).arg(output_dir);
                if !verbose {
                    cmd.arg("-i");
                } // Disable prompts
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
            _ => anyhow::bail!("Unsupported ARJ tool: {}", bin_name),
        }
        Ok(cmd)
    }
}
