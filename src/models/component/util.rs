use crate::errors::ServiceError;
use diesel::prelude::*;
use uuid::Uuid;

/// checking whether the component has flag is_standard
pub fn check_is_standard(
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> Result<bool, ServiceError> {
    use crate::schema::component_ref::dsl::*;

    let get_component_status: bool = component_ref
        .filter(uuid.eq(target_component_uuid))
        .select(is_standard)
        .first(conn)
        .unwrap_or(false);

    match get_component_status {
        true => Ok(true),
        // false => Ok(false),
        _ => Err(ServiceError::BadRequest(
            "The component is not standard.".to_string(),
        )),
    }
}
