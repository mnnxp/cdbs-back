use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::util::get_component_by_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Get component uuid from fileset by uuid
pub fn get_component_by_fileset(
    target_fileset_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    // get component uuid
    get_component_by_modification(
        // get component modification uuid from fileset data
        &get_modification_by_fileset(target_fileset_uuid, conn)?,
        conn
    )
}

/// Get modification uuid from fileset by uuid
pub fn get_modification_by_fileset(
    target_fileset_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    use crate::schema::fileset_for_program::dsl::*;

    let get_modification_uuid = fileset_for_program
        .filter(uuid.eq(target_fileset_uuid))
        .select(modification_uuid)
        .first::<Uuid>(conn);

    match get_modification_uuid {
        Ok(mn_uuid) => Ok(mn_uuid),
        _ => Err(ServiceError::BadRequest(
            "Not found fileset data".to_string(),
        )),
    }
}
