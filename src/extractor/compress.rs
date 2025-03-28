use super::Extractor;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct CompressExtractor;

impl Extractor for CompressExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["Z"]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["uncompress"]
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

        let target_file = output_dir.join(file.file_name().unwrap());
        std::fs::copy(file, &target_file)?;

        cmd.current_dir(output_dir);
        cmd.arg(target_file);

        if verbose {
            println!("Extracting {} to {}", file.display(), output_dir.display());
        }

        Ok(cmd)
    }
}
