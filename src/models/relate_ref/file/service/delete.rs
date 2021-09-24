use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::relate_ref::file::model::SlimFile;
use crate::models::relate_ref::file::util::check_file_owner;
use crate::storage::model::StorageAccess;
use crate::storage::delete::delete_object;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete files records in database by uuid
pub(crate) fn delete_row_by_uuid(
    delete_file_uuids: &Uuid,
    conn: &PgConnection
) -> i32 {
    use crate::schema::file_ref::dsl::*;

    diesel::delete(
        file_ref.filter(uuid.eq(delete_file_uuids))
    ).execute(conn).unwrap_or_default() as i32
}

/// Delete file to storage and row in database
pub(crate) async fn delete_file_by_uuid(
    target_user_uuid: &Uuid,
    file_uuid: &Uuid,
    pool: PgPool,
) -> ServiceResult<i32> {
    let conn = pool.get().unwrap();

    // getting SlimFile data for get file path
    let slim_file = SlimFile::get_file_by_uuid(
        file_uuid,
        &conn,
    )?;

    // getting storage access data for target user
    let storage_access = StorageAccess::get(&conn)?;

    // ownership check and data update
    if check_file_owner(
        target_user_uuid,
        &slim_file.uuid,
        &conn,
    ) == 1 {
        // delete file in storage
        let del_on_storage = delete_object(
            &storage_access,
            &slim_file.path_file,
        ).await;

        if del_on_storage {
            let result_del_row = delete_row_by_uuid(
                &slim_file.uuid,
                &conn
            );

            debug!("Removing completed: {:?}", result_del_row);

            return Ok(result_del_row)
        } else {
            debug!("Removing failed: {:?}", slim_file);
        }
    }

    Err(ServiceError::BadRequest("Unsuccessful delete data".to_string()))
}
