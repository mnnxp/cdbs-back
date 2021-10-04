use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Checking whether the component has flag is_base
/// return true or false
pub fn check_is_base(
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let get_component_status = component_ref
        .filter(uuid.eq(target_component_uuid))
        .select(is_base)
        .get_result::<bool>(conn);

    match get_component_status {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}

/// Checking whether the component has flag is_base
/// return err if not base
pub fn check_is_base_with_err(
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
