use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct IsoExtractor;

impl Extractor for IsoExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["iso"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["7z", "bsdtar"]
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let mut cmd = Command::new(binary);
        if binary.ends_with("7z") {
            cmd.arg("x")
                .arg("-y")
                .arg(format!("-o{}", output_dir.display()))
                .arg(file);
            if !verbose {
                cmd.arg("-bso0").arg("-bd");
            }
        } else {
            cmd.arg("-xf").arg(file).arg("-C").arg(output_dir);
            if verbose {
                cmd.arg("-v");
            }
        }
        Ok(cmd)
    }
}
