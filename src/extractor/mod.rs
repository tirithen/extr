use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::process::Command;
use std::sync::RwLock;

use crate::extractor::arj::ArjExtractor;
use crate::extractor::bzip2::Bzip2Extractor;
use crate::extractor::cab::CabExtractor;
use crate::extractor::compress::CompressExtractor;
use crate::extractor::gzip::GzipExtractor;
use crate::extractor::iso::IsoExtractor;
use crate::extractor::lha::LhaExtractor;
use crate::extractor::lzip::LzipExtractor;
use crate::extractor::lzma::LzmaExtractor;
use crate::extractor::lzop::LzopExtractor;
use crate::extractor::pkg::DebRpmExtractor;
use crate::extractor::rar::RarExtractor;
use crate::extractor::sevenz::SevenzExtractor;
use crate::extractor::sfx::SfxExtractor;
use crate::extractor::tar::TarExtractor;
use crate::extractor::xz::XzExtractor;
use crate::extractor::zip::ZipExtractor;
use crate::extractor::zstd::ZstdExtractor;
use crate::trust::{is_trusted_bin_path, run_command};

pub mod arj;
pub mod bzip2;
pub mod cab;
pub mod compress;
pub mod gzip;
pub mod iso;
pub mod lha;
pub mod lzip;
pub mod lzma;
pub mod lzop;
pub mod pkg;
pub mod rar;
pub mod sevenz;
pub mod sfx;
pub mod tar;
pub mod xz;
pub mod zip;
pub mod zstd;

pub fn register_extractors() {
    register_extractor(&ArjExtractor);
    register_extractor(&Bzip2Extractor);
    register_extractor(&CabExtractor);
    register_extractor(&CompressExtractor);
    register_extractor(&GzipExtractor);
    register_extractor(&IsoExtractor);
    register_extractor(&LhaExtractor);
    register_extractor(&LzipExtractor);
    register_extractor(&LzmaExtractor);
    register_extractor(&LzopExtractor);
    register_extractor(&DebRpmExtractor);
    register_extractor(&RarExtractor);
    register_extractor(&SevenzExtractor);
    register_extractor(&SfxExtractor);
    register_extractor(&TarExtractor);
    register_extractor(&XzExtractor);
    register_extractor(&ZipExtractor);
    register_extractor(&ZstdExtractor);
}

pub trait Extractor: Send + Sync {
    fn file_extensions(&self) -> Vec<&str>;

    fn binary_names(&self) -> Vec<&'static str>;

    fn get_verified_binary(&self) -> Result<std::path::PathBuf> {
        self.binary_names()
            .into_iter()
            .find_map(|binary| {
                if let Ok(path) = which::which(binary) {
                    if is_trusted_bin_path(&path) {
                        return Some(path);
                    }
                    None
                } else {
                    None
                }
            })
            .context(format!(
                "💥 Whoops! No suitable extraction tool found on your system for {} archives. Please use your package manager to install one of: {}",
                self.file_extensions().join(", "),
                self.binary_names().join(", ")
            ))
    }

    fn build_command(
        &self,
        binary: &Path,
        file: &Path,
        output_dir: &Path,
        verbose: bool,
    ) -> Result<Command>;

    fn extract(&self, file: &Path, output_dir: &Path, verbose: bool) -> Result<()> {
        let binary = self.get_verified_binary()?;
        let cmd = self.build_command(&binary, file, output_dir, verbose)?;
        std::fs::create_dir_all(output_dir)?;
        run_command(cmd, verbose)
    }
}

lazy_static! {
    static ref EXTRACTORS: RwLock<HashMap<&'static str, &'static dyn Extractor>> =
        RwLock::new(HashMap::new());
}

pub fn get_health_info() -> Vec<(String, Vec<(String, bool)>)> {
    let extractors = EXTRACTORS.read().unwrap();
    let mut formats = BTreeMap::new();

    for (ext, extractor) in extractors.iter() {
        let binaries = extractor
            .binary_names()
            .into_iter()
            .map(|bin| {
                let exists = which::which(bin)
                    .map(|path| is_trusted_bin_path(&path))
                    .unwrap_or(false);
                (bin.to_string(), exists)
            })
            .collect();
        formats.insert(ext.to_string(), binaries);
    }

    formats.into_iter().collect()
}

pub fn register_extractor(extractor: &'static dyn Extractor) {
    let mut extractors = EXTRACTORS.write().unwrap();
    for ext in extractor.file_extensions() {
        extractors.insert(ext, extractor);
    }
}

pub fn get_extractor(path: &Path) -> Option<&'static dyn Extractor> {
    let extension = get_extension(path)?;
    let extractors = EXTRACTORS.read().unwrap();
    extractors.get(extension.as_str()).copied()
}

fn get_extension(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_str()?;
    let parts: Vec<&str> = file_name.split('.').collect();

    match parts.as_slice() {
        [.., "tar", last] => Some(format!("{}.{}", "tar", last).to_lowercase()),
        [.., last] => Some(last.to_lowercase()),
        _ => None,
    }
}
