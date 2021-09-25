use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// checking whether the component has flag is_standard
pub fn check_is_standard(
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let get_component_status = component_ref
        .filter(uuid.eq(target_component_uuid)
        .and(is_standard.eq(true)))
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

/// Checking onwed component
/// Return error if user not owned
pub fn check_is_owned(
    target_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    let check_owner_component = component_ref
        .filter(user_uuid.eq(target_user_uuid)
        .and(uuid.eq(target_component_uuid)))
        .limit(1)
        .execute(conn);

    match check_owner_component {
        Ok(count) if count == 1 => Ok(true),
        Ok(_) => Err(ServiceError::BadRequest(
            "Access denied".to_string(),
        )),
        // Ok(_) => Ok(false),
        _ => Err(ServiceError::BadRequest(
            "Failed check data".to_string(),
        )),
    }
}
