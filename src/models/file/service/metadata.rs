use std::fs;
use crate::models::file::util as file;
use diesel::prelude::*;
// use crate::errors::ServiceResult;

#[derive(Debug, Deserialize)]
pub(crate) struct ParseFileData {
    pub hash: Vec<u8>,
    pub id_ext: i32,
    pub filesize: i32,
}

pub(crate) fn metadata(
    filepath: &str,
    filename: &str,
    conn: &PgConnection
) -> ParseFileData {
    // calculated blake3 for hash
    let hash = file::calculate_blake3(filepath);

    // get id for extension
    let id_ext = file::find_id_ext(filename, conn);

    // get metadata for filesize
    let size = fs::metadata(filepath).unwrap().len() as i32;
    // let metadata = fs::metadata(filepath).unwrap();
    // debug!("METADATA TEST FILE {:?}", metadata);

    ParseFileData {
        filesize: (size),
        id_ext: (id_ext),
        hash: (hash)
    }
}
