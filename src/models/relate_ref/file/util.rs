// use std::fs;
use regex::Regex;
use diesel::prelude::*;
use uuid::Uuid;
// use crate::errors::ServiceResult;

// #[derive(Debug, Deserialize)]
// pub(crate) struct ParseFileData {
//     pub hash: Vec<u8>,
//     pub id_ext: i32,
//     pub filesize: i32,
// }

// /// Function for calculate and get metadata for file
// pub(crate) fn metadata(
//     filepath: &str,
//     filename: &str,
//     conn: &PgConnection
// ) -> ParseFileData {
//     // calculated blake3 for hash
//     let hash = calculate_blake3(filepath);
//
//     // get id for extension
//     let id_ext = find_id_ext(filename, conn);
//
//     // get metadata for filesize
//     let size = fs::metadata(filepath).unwrap().len() as i32;
//
//     ParseFileData {
//         filesize: (size),
//         id_ext: (id_ext),
//         hash: (hash)
//     }
// }

// /// Calculate Blake3 hash of file
// pub(crate) fn calculate_blake3(path: &str) -> Vec<u8> {
//     let file = fs::read(path).unwrap();
//     // let hash = blake3::hash(&file);
//     // debug!("Hash value: {:#?}", hash);
//
//     blake3::hash(&file).as_bytes().to_vec()
// }

/// Find extension id on table for file extension
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

/// Checking pre file data for the user
pub(crate) fn check_write_data(
    user_uuid: &Uuid,
    path_file: &str,
    conn: &PgConnection
) -> bool {
    use crate::schema::file_ref::dsl as file_ref;

    // find component this modification by uuid
    let search_result = file_ref::file_ref
        .filter(file_ref::path_file.eq(path_file))
        .filter(file_ref::user_uuid.eq(user_uuid))
        .execute(conn).unwrap_or(0);

    matches!(search_result, 1)
}
