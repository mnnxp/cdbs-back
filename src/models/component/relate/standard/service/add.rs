use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::standard::model::{
    IptStandardToComponentData, InsertableStandardToComponent
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::standard_to_component::dsl as standard_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Add related standard for component
/// insert row in standard_to_component table
pub(crate) fn add_standard_to_component(
    logged_user_uuid: &Uuid,
    data: &IptStandardToComponentData,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    let found_standard = standard_to_component::standard_to_component
        .filter(standard_to_component::component_uuid.eq(data.component_uuid)
        .and(standard_to_component::standard_uuid.eq(data.standard_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check standards component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if found_standard > 0 {
        return Err(ServiceError::BadRequest(
            "This standard is already associated with the component".to_string()
        ))
    }

    let new_component_standard: InsertableStandardToComponent = data.into();

    diesel::insert_into(standard_to_component::standard_to_component)
        .values(&new_component_standard)
        .returning(standard_to_component::component_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed add standard component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(true)
}
