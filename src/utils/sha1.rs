use sha1::{Digest, Sha1};
use std::fs::File;
use std::path::Path;

use crate::errors::GrindstoneResult;

pub fn get_sha1(file: impl AsRef<Path>) -> GrindstoneResult<Vec<u8>> {
    let mut file = File::open(&file)?;
    let mut hasher = Sha1::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    Ok(hash.to_vec())
}
