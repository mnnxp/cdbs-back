use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::util::check_default_file;
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

use super::update::related_file_updated_at;

/// Deletes a file in storage and row in database
/// with check ownership by uuid
pub(crate) fn delete_file_with_check_by_uuid(
    logged_user_uuid: &Uuid,
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // ownership check and data update
    check_file_owner_err(logged_user_uuid, file_uuid, conn)?;
    // set flag is_delete for target file
    let res = delete_file_by_uuid(file_uuid, conn)?;
    if res {
        // save info about the action in related object and logs
        related_file_updated_at(logged_user_uuid, file_uuid, None, true, conn)?;
    }
    Ok(res)
}

/// Set flag is_delete for delete data in future
/// without check access for logged user
pub(crate) fn delete_file_by_uuid(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    if check_default_file(file_uuid) {
        // this default file
        return Ok(false);
    }

    diesel::update(file_ref::file_ref)
        .filter(file_ref::uuid.eq(file_uuid))
        .set((
            file_ref::is_checked.eq(false),
            file_ref::is_hidden.eq(true),
            file_ref::is_delete.eq(true),
        ))
        .returning(file_ref::is_delete)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failded set delete flag database : {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Set flags is_delete for delete data in future
/// without check access for logged user
pub(crate) fn delete_file_by_uuids(
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let count = diesel::update(file_ref::file_ref)
        .filter(file_ref::uuid.eq_any(file_uuids))
        .set((
            file_ref::is_checked.eq(false),
            file_ref::is_hidden.eq(true),
            file_ref::is_delete.eq(true),
        ))
        .execute(conn)
        .map_err(|err| {
            debug!("Failded set delete flags database : {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count > 0)
}
