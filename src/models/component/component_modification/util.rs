use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref ROOT_MODIFICATION_UUID : Uuid =
        Uuid::parse_str("aba22d59-4f6c-44a4-9a37-2d38f0e577a8")
            .expect("Set default image uuid failed!");
}

/// Retund default component modification uuid
pub(crate) fn get_root_modification_uuid() -> Uuid {
    *ROOT_MODIFICATION_UUID
}

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
