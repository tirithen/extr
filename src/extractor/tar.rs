use std::{path::Path, process::Command};

use anyhow::Result;

use super::Extractor;

pub struct TarExtractor;

impl Extractor for TarExtractor {
    fn file_extensions(&self) -> Vec<&'static str> {
        vec![
            "tar", "tar.gz", "tgz", "tar.bz2", "tbz2", "tar.xz", "txz", "tar.zst",
        ]
    }

    fn binary_names(&self) -> Vec<&'static str> {
        vec!["unar", "tar", "gtar", "bsdtar"]
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
            "unar" => {
                cmd.arg("-o").arg(output_dir).arg(file);
                if !verbose {
                    cmd.arg("-q");
                }
            }
            _ => {
                cmd.arg("-xf").arg(file).arg("-C").arg(output_dir);

                if verbose {
                    cmd.arg("-v");
                }

                // Add compression flag based on file extension
                if let Some(ext) = file.extension() {
                    match ext.to_str().unwrap_or("") {
                        "gz" | "tgz" => {
                            cmd.arg("-z");
                        }
                        "bz2" | "tbz2" => {
                            cmd.arg("-j");
                        }
                        "xz" | "txz" => {
                            cmd.arg("-J");
                        }
                        "zst" => {
                            cmd.arg("--zstd");
                        }
                        "" => {}
                        _ => {
                            anyhow::bail!(
                                "Unsupported tar extension tar.{}",
                                ext.to_string_lossy()
                            );
                        }
                    };
                }
            }
        }

        Ok(cmd)
    }
}
