use std::fs;
// use std::os::unix::fs::MetadataExt;
// use std::fs::Metadata;
// use crate::errors::ServiceResult;
use sha2::{Sha256, Digest};

#[derive(Debug, Deserialize)]
pub struct ParseFileData {
    pub hash: Vec<u8>,
    pub id_ext: i32,
    pub filesize: i32,
}

pub(crate) fn metadata(filepath: &str) -> ParseFileData {
    // calculated Sha256 for hash
    let mut hasher = Sha256::new();
    hasher.update(filepath);
    let hash = hasher.finalize();

    // get metadata for filesize
    let metadata = fs::metadata(filepath).unwrap();
    // debug!("METADATA TEST FILE {:?}", metadata);
    // let size = metadata.size() as f64;
    let size = metadata.len() as i32;

    ParseFileData {
        filesize: (size),
        id_ext: (1), // fix this after
        hash: (hash.to_vec())
    }
}
