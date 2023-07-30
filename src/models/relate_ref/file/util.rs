use crate::errors::{ServiceResult, ServiceError};
use regex::Regex;
use diesel::prelude::*;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref DEFAULT_IMAGE_UUID : Uuid =
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")
            .expect("Set default image uuid failed!");
}

/// Retund default image uuid
pub(crate) fn get_default_image() -> Uuid {
    *DEFAULT_IMAGE_UUID
}

/// Check default file by uuid
pub(crate) fn check_default_file(file_uuid: &Uuid) -> bool {
    let defalt_uuid = *DEFAULT_IMAGE_UUID;
    &defalt_uuid == file_uuid
}

/// Find extension id on table for file extension
pub(crate) fn find_id_ext(
    filename: &str,
    conn: &mut PgConnection
) -> i32 {
    use crate::schema::extension_ref::dsl::*;
    // debug!("Filename_str {:?}", filename);
    let ext_str =
        Regex::new(r"\.\w+$")
            .unwrap()
            .find(filename)
            .map(|m| m.as_str())
            .unwrap_or_default();
    // debug!("Ext_str {:?}", ext_str);
    if ext_str.is_empty() {
        return 1
    }
    // find id extension or set not found id = 1
    extension_ref
        .filter(extension.eq(ext_str))
        .select(id)
        .first::<i32>(conn).unwrap_or(1)
}

/// Checking if a file is owned and not checked or deleted
pub(crate) fn check_write_data(
    user_uuid: &Uuid,
    path_file: &str,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    use crate::schema::file_ref::dsl as file_ref;

    let check_result = file_ref::file_ref
        .filter(file_ref::path_file.eq(path_file)
            .and(file_ref::user_uuid.eq(user_uuid)
            .and(file_ref::is_checked.eq(false)
            .and(file_ref::is_delete.eq(false)))))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get object headers: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(check_result == 1)
}

/// Checking that the file name matches the image
pub(crate) fn check_image_filename(filename: &str) -> bool {
    let ext_str = Regex::new(r"\.\w+$").unwrap().find(filename).unwrap().as_str();

    matches!(
        ext_str.to_lowercase().as_str(),
        ".apng" | ".avif" | ".gif" |
        ".jpg" | ".jpeg" | ".jpe" |
        ".jif" | ".jfif" | ".png" |
        ".svg" | ".webp"
    )
}
