use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::FileData;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn update_file_data_by_name(
    user_uuid: &Uuid,
    path_file: &str,
    new_file_data: &FileData,
    ownership_check: bool,
    conn: &PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::file_ref::dsl as file_ref;

    let target_file_uuid: Uuid;

    // user non-ownership can have access,
    // so ownership verification is not always necessary
    if ownership_check {
        target_file_uuid = file_ref::file_ref
            .filter(file_ref::uuid_user.eq(user_uuid)
            .and(file_ref::path_file.eq(path_file)))
            .select(file_ref::uuid)
            .first(conn).unwrap_or_default();
    } else {
        target_file_uuid = file_ref::file_ref
            .filter(file_ref::path_file.eq(path_file))
            .select(file_ref::uuid)
            .first(conn).unwrap_or_default();
    }

    if target_file_uuid.is_nil() {
        return Err(ServiceError::BadRequest("Not found target file".to_string()))
    }

    let mut count_update_columns = 0;

    if let Some(value) = new_file_data.uuid_file_parent {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::uuid_file_parent.eq(value))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = &new_file_data.hash {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::hash.eq(value.to_vec()))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = new_file_data.uuid_user {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::uuid_user.eq(value))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = &new_file_data.filename {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::filename.eq(value.clone()))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = &new_file_data.content_type {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::content_type.eq(value.clone()))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = new_file_data.id_ext {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::id_ext.eq(value))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = new_file_data.filesize {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::filesize.eq(value))
            .execute(conn).unwrap_or_default() as i32;
    }
    if let Some(value) = &new_file_data.path_file {
        count_update_columns += diesel::update(file_ref::file_ref
            .filter(file_ref::uuid.eq(&target_file_uuid)))
            .set(file_ref::path_file.eq(value.clone()))
            .execute(conn).unwrap_or_default() as i32;
    }

    // new date for updated_at in file_ref table
    count_update_columns += diesel::update(file_ref::file_ref
        .filter(file_ref::uuid.eq(&target_file_uuid)))
        .set(file_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn).unwrap_or_default() as i32;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
