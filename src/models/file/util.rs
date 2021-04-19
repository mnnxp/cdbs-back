use std::fs;
use regex::Regex;
use diesel::prelude::*;
// use crate::errors::ServiceResult;

pub(crate) fn calculate_blake3(path: &str) -> Vec<u8> {
    let file = fs::read(path).unwrap();
    // let hash = blake3::hash(&file);
    // debug!("Hash value: {:#?}", hash);

    blake3::hash(&file).as_bytes().to_vec()
}

pub(crate) fn find_id_ext(filename: &str, conn: &PgConnection) -> i32 {
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
