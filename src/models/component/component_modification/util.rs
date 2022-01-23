use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Get component uuid from modification by uuid
pub(crate) fn get_component_by_modification(
    target_modification_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    use crate::schema::component_modification_list::dsl::*;

    let get_component_uuid = component_modification_list
        .filter(uuid.eq(target_modification_uuid))
        .select(component_uuid)
        .first::<Uuid>(conn);

    match get_component_uuid {
        Ok(ct_uuid) => Ok(ct_uuid),
        _ => Err(ServiceError::BadRequest(
            "Not found modification data".to_string(),
        )),
    }
}
