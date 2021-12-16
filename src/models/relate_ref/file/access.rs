use crate::errors::{ServiceResult, ServiceError};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Find and check existence user owned component
/// return err if not found file
pub(crate) fn check_file_owner_err(
    user_uuid: &Uuid,
    file_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    // find file with target user
    let filesize: i64 = file_ref::file_ref
        .filter(file_ref::uuid.eq(file_uuid)
        .and(file_ref::user_uuid.eq(user_uuid)))
        .select(file_ref::filesize)
        .first(conn)
        .map_err(|err| {
            debug!("Not found file: {:?}", err);
            ServiceError::BadRequest("Access denied".to_string())
        })?;

    Ok(filesize > 0)
}
