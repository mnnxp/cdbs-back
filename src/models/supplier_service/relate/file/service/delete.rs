use crate::errors::{ServiceResult, ServiceError};
use crate::models::supplier_service::relate::file::model::DeleteServiceFileData;
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::schema::file_to_service::dsl as file_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Deletes service files
pub(crate) fn delete_service_file(
    arguments: &DeleteServiceFileData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &arguments.service_uuid,
        &need_access_level,
        conn,
    )?;

    let del_file = diesel::delete(file_to_service::file_to_service)
        .filter(file_to_service::service_uuid.eq(&arguments.service_uuid)
        .and(file_to_service::file_uuid.eq(&arguments.file_uuid)))
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
