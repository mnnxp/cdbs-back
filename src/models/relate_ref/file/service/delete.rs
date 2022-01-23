use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::access::check_file_owner_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete file in storage and row in database
/// with check ownership by uuid
pub(crate) fn delete_file_with_check_by_uuid(
    logged_user_uuid: &Uuid,
    file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // ownership check and data update
    check_file_owner_err(logged_user_uuid, file_uuid, conn)?;

    // set flag is_delete for target file
    delete_file_by_uuid(file_uuid, conn)
}

/// Set flag is_delete for delete data in future
/// without check access for logged user
pub(crate) fn delete_file_by_uuid(
    file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::file_ref::dsl as file_ref;

    diesel::update(file_ref::file_ref)
        .filter(file_ref::uuid.eq(file_uuid))
        .set(file_ref::is_delete.eq(true))
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
    file_uuid: &[Uuid],
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::file_ref::dsl as file_ref;

    let count = diesel::update(file_ref::file_ref)
        .filter(file_ref::uuid.eq_any(file_uuid))
        .set(file_ref::is_delete.eq(true))
        .execute(conn)
        .map_err(|err| {
            debug!("Failded set delete flag database : {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count > 0)
}
