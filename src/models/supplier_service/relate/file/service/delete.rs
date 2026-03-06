use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::supplier_service::relate::file::model::DeleteServiceFileData;
use crate::models::supplier_service::service::update::change_service_updated_at;
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
        need_access_level,
        conn,
    )?;

    let del_file = diesel::delete(file_to_service::file_to_service)
        .filter(
            file_to_service::service_uuid
                .eq(&arguments.service_uuid)
                .and(file_to_service::file_uuid.eq(&arguments.file_uuid)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail delete row: {:?}", err);
            ServiceError::InternalServerError
        })?;
    if del_file == 0 {
        return Ok(false); // not found file
    }
    change_service_updated_at(
        &arguments.service_uuid,
        logged_user_uuid,
        format!("Deleted the file uuid: {:?}", &arguments.file_uuid),
        conn,
    )?;
    // set flag for delete file in storage
    delete_file_by_uuid(&arguments.file_uuid, conn)
}
