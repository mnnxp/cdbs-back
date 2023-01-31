use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    access::util::check_access_component_for_user,
    component_modification::relate::file::model::DelModificationFileData,
    component_modification::util::get_component_by_modification,
};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::schema::file_to_modification::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete row in file_to_modification table
/// without delete row in  file_ref table and file in storage
pub(crate) fn delete_modification_file(
    logged_user_uuid: &Uuid,
    data: &DelModificationFileData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&data.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let del_file = diesel::delete(file_to_modification)
        .filter(modification_uuid.eq(&data.modification_uuid)
        .and(file_uuid.eq(&data.file_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail delete row: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match del_file {
        // not found file
        0 => Ok(false),
        // set flag for delete file in storage
        _ => delete_file_by_uuid(&data.file_uuid, conn),
    }
}
