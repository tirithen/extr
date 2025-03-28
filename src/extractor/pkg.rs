use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct DebRpmExtractor;

impl Extractor for DebRpmExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["deb", "rpm"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["bsdtar", "ar", "dpkg", "rpm2cpio"]
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
            "bsdtar" => {
                cmd.arg("-xf").arg(file).arg("-C").arg(output_dir);
                if verbose {
                    cmd.arg("-v");
                }
            }
            "dpkg" => {
                cmd.arg("-x").arg(file).arg(output_dir);
            }
            "rpm2cpio" => {
                cmd.arg(file)
                    .arg("|")
                    .arg("cpio")
                    .arg("-idm")
                    .arg("-D")
                    .arg(output_dir);
            }
            "ar" => {
                cmd.arg("x").arg(file).arg("--output").arg(output_dir);
            }
            _ => anyhow::bail!("Unsupported package tool: {}", bin_name),
        }
        Ok(cmd)
    }
}
