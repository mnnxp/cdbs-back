use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::relate::file::model::DeleteStandardFileData;
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::file_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete row in file_to_standard table
/// without delete row in  file_ref table and file in storage
pub(crate) fn delete_standard_file(
    logged_user_uuid: &Uuid,
    arguments: &DeleteStandardFileData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &arguments.standard_uuid,
        &need_access_level,
        conn,
    )?;

    // delete only row in file_to_standard table
    match diesel::delete(file_to_standard)
        .filter(standard_uuid.eq(&arguments.standard_uuid)
        .and(file_uuid.eq(&arguments.file_uuid)))
        .execute(conn) {
        Ok(count) => {
            if count == 0 {
                return Ok(false)
            }
            debug!("Delete standard file row: {:?}", count);
            Ok(true)
        },
        Err(err) => {
            debug!("Fail delete row: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
