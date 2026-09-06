use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::models::standard::relate::file::model::DeleteStandardFileData;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет файл стандарта.
pub(crate) fn delete_standard_file(
    logged_user_uuid: &Uuid,
    arguments: &DeleteStandardFileData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        &arguments.standard_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let del_file = diesel::delete(file_to_standard::file_to_standard)
        .filter(
            file_to_standard::standard_uuid
                .eq(&arguments.standard_uuid)
                .and(file_to_standard::file_uuid.eq(&arguments.file_uuid)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail delete row: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match del_file {
        // not found file
        0 => Ok(false),
        // set flag for delete file in storage
        _ => delete_file_by_uuid(&arguments.file_uuid, conn),
    }
}
