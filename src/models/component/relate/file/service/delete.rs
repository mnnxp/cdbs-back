use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::relate::file::model::DelComponentFileData;
use crate::schema::file_to_component::dsl::*;
use diesel::prelude::*;
// use uuid::Uuid;

/// Delete row in file_to_component table
/// without delete row in  file_ref table and file in storage
pub(crate) fn delete_component_file(
    // target_user_uuid: &Uuid, <-- todo!(manager access)
    data: &DelComponentFileData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // delete only row in file_to_component table
    match diesel::delete(file_to_component)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(file_uuid.eq(&data.file_uuid)))
        .execute(conn) {
        Ok(count) => {
            if count == 0 {
                return Ok(false)
            }
            debug!("Delete component file row: {:?}", count);
            Ok(true)
        },
        Err(err) => {
            debug!("Fail delete row: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
