use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::relate::file::model::DelModificationFileData;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::schema::file_to_modification::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete row in file_to_modification table
/// without delete row in  file_ref table and file in storage
pub(crate) fn delete_modification_file(
    logged_user_uuid: &Uuid,
    data: &DelModificationFileData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&data.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // delete only row in file_to_modification table
    match diesel::delete(file_to_modification)
        .filter(modification_uuid.eq(&data.modification_uuid)
        .and(file_uuid.eq(&data.file_uuid)))
        .execute(conn) {
        Ok(count) => {
            if count == 0 {
                return Ok(false)
            }
            debug!("Delete modification file row: {:?}", count);
            Ok(true)
        },
        Err(err) => {
            debug!("Fail delete row: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
