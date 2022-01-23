use crate::errors::{ServiceResult, ServiceError};
use regex::Regex;
use diesel::prelude::*;
use uuid::Uuid;

/// Find extension id on table for file extension
pub(crate) fn find_id_ext(
    filename: &str,
    conn: &PgConnection
) -> i32 {
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
) -> ServiceResult<bool> {
    use crate::schema::file_ref::dsl as file_ref;

    let check_result = file_ref::file_ref
        .filter(file_ref::path_file.eq(path_file))
        .filter(file_ref::user_uuid.eq(user_uuid))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get object headers: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_result == 1)
}
