use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct LzopExtractor;

impl Extractor for LzopExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["lzo"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["lzop"]
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

        cmd.arg("-d");
        cmd.arg("-p");
        cmd.arg(output_dir);
        cmd.arg("-k");

        if verbose {
            cmd.arg("-v");
        }

        cmd.arg(file);

        Ok(cmd)
    }
}
