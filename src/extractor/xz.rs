use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct XzExtractor;

impl Extractor for XzExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["xz"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["xz", "unxz"]
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command> {
        let mut cmd = Command::new(binary);

        std::fs::create_dir_all(output_dir)?;

        if binary.ends_with("xz") {
            cmd.arg("-d");
        }

        cmd.arg("-k"); // Keep original file

        if verbose {
            cmd.arg("-v");
        }

        cmd.current_dir(output_dir);
        cmd.arg(file);

        Ok(cmd)
    }
}
