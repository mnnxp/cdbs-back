use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::relate::file::model::DelComponentFileData;
use crate::schema::file_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete row in file_to_component table
pub(crate) fn delete_component_file(
    logged_user_uuid: &Uuid,
    data: &DelComponentFileData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    let count = diesel::delete(file_to_component)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(file_uuid.eq(&data.file_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail delete row: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // todo!(here delete files of file_ref table and of storage)

    Ok(count > 0)
}
