use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// checking whether the component has flag is_base
pub fn check_is_base(
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let get_component_status = component_ref
        .filter(uuid.eq(target_component_uuid)
        .and(is_base.eq(true)))
        .limit(1)
        .execute(conn);

    match get_component_status {
        Ok(count) if count == 1 => Ok(true),
        Ok(_) => Err(ServiceError::BadRequest(
            "The component is not standard.".to_string(),
        )),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}
