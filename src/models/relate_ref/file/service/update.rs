use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::component::access::util::check_is_owner_with_err as component_check_is_owner_with_err;
use crate::models::component::component_modification::{
    fileset_for_program::util::get_component_by_fileset,
    util::get_component_by_modification,
};
use crate::models::relate_ref::file::{
    model::{FileData, SlimFile, ListObject},
    repository::get_filename_hidden_rev_by_uuid,
    util::{check_write_data, detect_relation_to_object, get_latest_file_revision, set_hidden_flag},
};
use crate::models::standard::access::util::check_is_owner_with_err as standard_check_is_owner_with_err;
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
) -> ServiceResult<usize> {
    let mut conn = pool.get().unwrap();

    let mut confirm_files: usize = 0;

    // getting SlimFile data for get files paths
    let slim_files = SlimFile::get_not_checked_by_uuids(
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
                    content_type: file_h.content_type,
                    filesize: file_h.content_length,
                    is_checked: false,
                    is_hidden: false,
                },
                true, // <- confirming upload file only by the same user who requested the upload url
                &mut conn,
            )?;

            debug!("Update rows: {:?}", update_file_rows);

            confirm_files += 1;
        }
    }

    match confirm_files == file_uuids.len() {
        true => Ok(confirm_files),
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
) -> ServiceResult<usize> {
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

    // new date for updated_at in file_ref table if update more one column
    if count_update_columns > 0 {
        diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set((
                file_ref::is_checked.eq(new_file_data.is_checked),
                file_ref::is_hidden.eq(new_file_data.is_hidden),
                file_ref::updated_at.eq(chrono::Local::now().naive_local()),
            ))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::InternalServerError
            })?;
        // return count of updates if there are more than 0
        return Ok(count_update_columns)
    }

    // return error if new data not different with old data
    Err(ServiceError::BadRequest("The data has already".to_string()))
}

/// Changes the active file version for an object, checking object ownership to user
pub(crate) fn set_active_revision_by_uuid(
    user_uuid: &Uuid,
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // determine a object associated with the file and enable revisions for the object
    let relate_object = detect_relation_to_object(file_uuid, conn)?;
    // check ownership to object
    match &relate_object {
        ListObject::Component(component_uuid) => {
            component_check_is_owner_with_err(user_uuid, component_uuid, conn)?;
        },
        ListObject::ComponentModification(modification_uuid) => {
            component_check_is_owner_with_err(
                user_uuid,
                &get_component_by_modification(modification_uuid, conn)?,
                conn
            )?;
        },
        ListObject::ComponentModificationSet(fileset_uuid) => {
            component_check_is_owner_with_err(
                user_uuid,
                &get_component_by_fileset(fileset_uuid, conn)?,
                conn
            )?;
        },
        ListObject::Standard(standard_uuid) => {
            standard_check_is_owner_with_err(user_uuid, standard_uuid, conn)?;
        },
        _not_match => {
            debug!("This file does not require versioning");
            return Err(ServiceError::BadRequest("File to object association not found".to_string()))
        },
    }
    // find active revision
    let filename = get_filename_hidden_rev_by_uuid(file_uuid, conn)
        .map_err(|err| {
            debug!("File already active or delete: {:?}", err);
            ServiceError::BadRequest("Revision already active or deleted".to_string())
        })?;
    match get_latest_file_revision(&relate_object, &filename, conn)? {
        Some((ref current_revision_uuid, _)) => {
            // change flag for hidden revision of file to active (is_hidden=false)
            if set_hidden_flag(file_uuid, false, conn)? {
                // after successful showing target revision, to hidden current revision of file (is hidden=true)
                set_hidden_flag(current_revision_uuid, true, conn)
            } else {
                Ok(false)
            }
        },
        None => {
            debug!("No active file revision found for: {:?}", filename);
            Err(ServiceError::BadRequest("No active file revision found".to_string()))
        },
    }
}