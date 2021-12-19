use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::relate_ref::file::model::SlimFile;
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::storage::model::StorageAccess;
use crate::storage::delete::delete_object;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete file record in database by uuid
pub(crate) fn delete_row_by_uuid(
    delete_file_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::file_ref::dsl::*;

    let del_count = diesel::delete(file_ref)
        .filter(uuid.eq(delete_file_uuid))
        .execute(conn)
        .map_err(|err| {
            debug!("Failded delete file record in database : {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(del_count as i32)
}

// /// Delete files records in database by uuid
// pub(crate) fn delete_rows_by_uuids(
//     delete_files_uuids: &[Uuid],
//     conn: &PgConnection
// ) -> i32 {
//     use crate::schema::file_ref::dsl::*;
//
//     diesel::delete(
//         file_ref.filter(uuid.eq_any(delete_files_uuids))
//     ).execute(conn).unwrap_or_default() as i32
// }

/// Delete file in storage and row in database
/// with check ownership by uuid
pub(crate) async fn delete_file_with_check_by_uuid(
    logged_user_uuid: &Uuid,
    file_uuid: &Uuid,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    // getting SlimFile data for get file path
    let slim_file = SlimFile::get_file_by_uuid(
        file_uuid,
        &conn,
    )?;

    // ownership check and data update
    if check_file_owner_err(logged_user_uuid, &slim_file.uuid, &conn)? {
        // delete file and data about file
        full_delete_file(&slim_file, pool).await
    } else {
        Err(ServiceError::BadRequest(
           "Unsuccessful delete data".to_string()
       ))
    }
}

/// Delete file to storage and row in database
pub(crate) async fn delete_file_by_uuid(
    file_uuid: &Uuid,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    // getting SlimFile data for get file path
    let slim_file = SlimFile::get_file_by_uuid(
        file_uuid,
        &conn,
    )?;

    full_delete_file(&slim_file, pool).await
}

/// Delete file to storage and row in database
/// Warning: this function without check access
pub(crate) async fn full_delete_file(
    slim_file: &SlimFile,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    // delete file in storage
    let res_del = delete_file_by_path(&slim_file.path_file, pool).await?;
    match res_del {
        true => {
            // delete rows about file
            let result_del_row = delete_row_by_uuid(
                &slim_file.uuid,
                &conn
            )?;

            match result_del_row < 1 {
                true => Err(ServiceError::BadRequest("Removing file info data failed".to_string())),
                false => {
                    debug!("Removing completed: {:?}", result_del_row);
                    Ok(true)
                },
            }
        },
        false => Err(ServiceError::BadRequest("Removing failed".to_string())),
    }
}

/// Delete file to storage
/// Warning: this function without check access
pub(crate) async fn delete_file_by_path(
    path_file: &str,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    // getting storage access data for target user
    let storage_access = StorageAccess::get(&conn)?;

    // delete file in storage
    Ok(delete_object(
        &storage_access,
        path_file,
    ).await)
}
