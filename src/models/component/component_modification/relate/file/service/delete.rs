use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::relate::file::model::DelModificationFileData;
use crate::schema::file_to_modification::dsl::*;
use diesel::prelude::*;
// use uuid::Uuid;

/// Delete row in file_to_modification table
/// without delete row in  file_ref table and file in storage
pub(crate) fn delete_modification_file(
    // target_user_uuid: &Uuid, <-- todo!(manager access)
    data: &DelModificationFileData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
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
