use std::{fs, path::PathBuf, time::SystemTime};

use eyre::{Ok, Result};
use lyre::{generate_hash, hashed_path, HASH_PATHS};

pub fn process() -> Result<()> {
    println!("Processing version hashes...");
    let start = SystemTime::now();

    for path in HASH_PATHS {
        let path: PathBuf = path.into();

        fs::write(hashed_path(&path), generate_hash(path)?)?;
    }

    println!("Processing version hashes took {:?}", start.elapsed()?);

    Ok(())
}
