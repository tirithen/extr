use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct CabExtractor;

impl Extractor for CabExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["cab"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["cabextract", "7z"]
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let mut cmd = Command::new(binary);
        if binary.ends_with("cabextract") {
            cmd.arg("-d").arg(output_dir).arg(file);
            if !verbose {
                cmd.arg("-q");
            }
        } else {
            // 7z
            cmd.arg("x")
                .arg("-y")
                .arg(format!("-o{}", output_dir.display()))
                .arg(file);
            if !verbose {
                cmd.arg("-bso0").arg("-bd");
            }
        }
        Ok(cmd)
    }
}
