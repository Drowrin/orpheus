use std::{
    collections::HashSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use colored::*;
use eyre::{Context, Result};
use sha2::{Digest, Sha256};

pub mod utils;

pub fn prepare() -> Result<()> {
    fs::create_dir_all("generated/repertoire/")?;
    Ok(())
}

pub trait Melody {
    fn name() -> &'static str;
    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>>;
    fn perform(parts: impl Iterator<Item = PathBuf>) -> Result<()>;

    fn generate_hash(path: &PathBuf) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(fs::read(&path).context(format!("Path: {:?}", path))?);

        Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
    }

    fn hash_path(path: &PathBuf) -> PathBuf {
        Path::new("./generated/repertoire/")
            .join(Self::name())
            .join(path.file_name().unwrap())
            .with_extension("hash")
    }

    fn read_hash(path: &PathBuf) -> Result<String> {
        Ok(fs::read_to_string(Self::hash_path(path))?)
    }

    fn write_hash(path: &PathBuf, hash: String) -> Result<()> {
        fs::write(Self::hash_path(path), hash)?;

        Ok(())
    }

    fn ready(path: &PathBuf) -> Result<bool> {
        let hash = match Self::read_hash(path) {
            Ok(sheet) => sheet,
            Err(_) => return Ok(false),
        };

        if Self::generate_hash(path)? != hash {
            return Ok(false);
        }

        Ok(true)
    }

    fn conduct() -> Result<()> {
        let needs_rebuild: Vec<PathBuf> = Self::source()?
            .into_iter()
            .map(|p| p.into())
            .filter(|p| {
                if let Ok(v) = Self::ready(p) {
                    return !v;
                }
                return true;
            })
            .collect();

        if needs_rebuild.len() == 0 {
            return Ok(());
        }

        print!("Rebuilding {}", Self::name());
        std::io::stdout().flush().unwrap();

        let path_set = Self::rendition()?
            .into_iter()
            .map(|p| p.into().with_file_name(""))
            .collect::<HashSet<_>>();
        for path in path_set {
            fs::create_dir_all(path)?;
        }

        let started = SystemTime::now();

        Self::perform(needs_rebuild.clone().into_iter())?;

        println!(
            " took {}",
            format!("{:?}", started.elapsed().unwrap()).yellow()
        );

        fs::create_dir_all(Path::new("./generated/repertoire/").join(Self::name()))?;
        for path in needs_rebuild {
            Self::write_hash(&path, Self::generate_hash(&path)?)?;
        }

        Ok(())
    }
}
