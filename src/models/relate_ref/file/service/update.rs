use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::relate_ref::file::{
    model::{FileData, SlimFile},
    util::check_write_data,
};
use crate::storage::model::StorageAccess;
use crate::storage::metadata::object_headers;
use diesel::prelude::*;
use uuid::Uuid;

/// Confirm upload file to storage
/// return number of column changes in database
pub(crate) async fn confirm_upload(
    target_user_uuid: &Uuid,
    file_uuids: &[Uuid],
    pool: &PgPool,
) -> ServiceResult<i32> {
    let mut conn = pool.get().unwrap();

    let mut confirm_files: usize = 0;

    // getting SlimFile data for get files paths
    let slim_files = SlimFile::get_by_files_uuids(
        file_uuids,
        &mut conn,
    ).unwrap();

    // getting storage access data for target user
    let storage_access = StorageAccess::from_env();

    // getting data for all files in vec
    for file_d in slim_files {
        // ownership check and data update
        if check_write_data(
            target_user_uuid,
            &file_d.path_file,
            &mut conn,
        )? {
            // todo!(getting metadata  by file id from client for validation)
            let file_h = object_headers(&storage_access, &file_d.path_file)
                .await
                .map_err(|err| {
                    debug!("Failed get object headers: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            // update file metadata in file_ref table
            let update_file_rows = update_file_data_by_uuid(
                target_user_uuid,
                &file_d.uuid,
                &FileData {
                    parent_file_uuid: None,
                    hash: None,
                    user_uuid: None,
                    filename: None,
                    content_type: file_h.content_type,
                    id_ext: None,
                    filesize: file_h.content_length,
                    path_file: None,
                },
                true, // <- confirming upload file only by the same user who requested the upload url
                &mut conn,
            )?;

            debug!("Upload completed: {:?}", update_file_rows);

            confirm_files += 1;
        }
    }

    match confirm_files == file_uuids.len() {
        true => Ok(confirm_files as i32),
        false => Err(ServiceError::BadRequest(
            "Unsuccessful check data".to_string()
        )),
    }
}

/// Update file data by uuid
/// without check access but with check owned
pub(crate) fn update_file_data_by_uuid(
    user_uuid: &Uuid,
    file_uuid: &Uuid,
    new_file_data: &FileData,
    ownership_check: bool,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::file_ref::dsl as file_ref;

    // user non-ownership can have access,
    // so ownership verification is not always necessary
    let mut query = file_ref::file_ref.into_boxed();
    query = match ownership_check {
        true => query.filter(file_ref::user_uuid.eq(user_uuid)
            .and(file_ref::uuid.eq(file_uuid))),
        false => query.filter(file_ref::uuid.eq(file_uuid)),
    };

    let target_file_uuid: Uuid = query
        .select(file_ref::uuid)
        .first(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if target_file_uuid.is_nil() {
        return Err(ServiceError::BadRequest("Not found target file".to_string()))
    }

    let mut count_update_columns = 0;

    if let Some(value) = new_file_data.parent_file_uuid {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::parent_file_uuid.ne(&value))))
            .set(file_ref::parent_file_uuid.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = &new_file_data.hash {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::hash.ne(&value))))
            .set(file_ref::hash.eq(value.to_vec()))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = new_file_data.user_uuid {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::user_uuid.ne(&value))))
            .set(file_ref::user_uuid.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = &new_file_data.filename {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::filename.ne(&value))))
            .set(file_ref::filename.eq(value.clone()))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = &new_file_data.content_type {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::content_type.ne(&value))))
            .set(file_ref::content_type.eq(value.clone()))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = new_file_data.id_ext {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::id_ext.ne(&value))))
            .set(file_ref::id_ext.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = new_file_data.filesize {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::filesize.ne(&value))))
            .set(file_ref::filesize.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }
    if let Some(value) = &new_file_data.path_file {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)
            .and(file_ref::path_file.ne(&value))))
            .set(file_ref::path_file.eq(value.clone()))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // new date for updated_at in file_ref table if update more one column
    if count_update_columns > 0 {
        diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::updated_at.eq(chrono::Local::now().naive_local()))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::InternalServerError
            })?;

        return Ok(count_update_columns as i32) // <- return count of updates if there are more than 0
    }

    // return error if new data not different with old data
    Err(ServiceError::BadRequest("The data has already".to_string()))
}
