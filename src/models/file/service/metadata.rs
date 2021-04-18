use std::fs;
// use std::os::unix::fs::MetadataExt;
// use std::fs::Metadata;
// use crate::errors::ServiceResult;
use regex::Regex;
use diesel::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ParseFileData {
    pub hash: Vec<u8>,
    pub id_ext: i32,
    pub filesize: i32,
}

pub(crate) fn metadata(filepath: &str, filename: &str, conn: &PgConnection) -> ParseFileData {

    // calculated Sha256 for hash (this is worked not correct :( )
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(filepath);
    let hash = hasher.finalize();

    // get id for extension
    let id_ext = find_id_ext(filename, conn);

    // get metadata for filesize
    let metadata = fs::metadata(filepath).unwrap();
    // debug!("METADATA TEST FILE {:?}", metadata);
    let size = metadata.len() as i32;

    ParseFileData {
        filesize: (size),
        id_ext: (id_ext),
        hash: (hash.to_vec())
    }
}


fn find_id_ext(filename: &str, conn: &PgConnection) -> i32 {
    use crate::schema::extension_ref::dsl::*;
    // debug!("Filename_str {:?}", filename);

    let ext_str = Regex::new(r"\w*$").unwrap().find(filename).unwrap().as_str();
    // debug!("Ext_str {:?}", ext_str);

    // find id extension or set not found id = 1
    extension_ref
        .filter(extension.eq(ext_str))
        .select(id)
        .first::<i32>(conn).unwrap_or(1)
}
