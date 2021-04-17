use std::fs;
use std::os::unix::fs::MetadataExt;
// use crate::errors::ServiceResult;

#[derive(Debug, Deserialize)]
pub struct ParseFileData {
    pub hash: String,
    pub id_ext: i32,
    pub filesize: f64,
}

pub(crate) fn metadata(filepath: &str) -> ParseFileData {

    let metadata = fs::metadata(filepath).unwrap();
    // debug!("METADATA TEST FILE {:?}", metadata);
    let size = metadata.size() as f64;

    ParseFileData {
        filesize: (size),
        id_ext: (1), // fix this after
        hash: ("000".to_owned())
    }
}
