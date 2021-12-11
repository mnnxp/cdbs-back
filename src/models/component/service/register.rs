use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    model::{IptComponentData, InsertableComponent, ComponentData},
    access::util::check_access_component_for_user,
};
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component(
    logged_user_uuid: &Uuid,
    data: &IptComponentData,
    conn: &PgConnection
) -> ServiceResult<Uuid> {

    let parent_component_uuid = match data.parent_component_uuid {
        Some(ref parent_component_uuid) => {
            check_access_component_for_user(
                logged_user_uuid,
                parent_component_uuid,
                &3, // need_access_level
                conn
            )?;
            *parent_component_uuid
        },
        None => Uuid::parse_str("a5953fd9-7393-4f1e-a899-06b5e159dbf1")?,
    };

    let component_data = ComponentData {
        parent_component_uuid,
        name: data.name.to_string(),
        description: data.description.to_string(),
        user_uuid: *logged_user_uuid,
        type_access_id: data.type_access_id,
        component_type_id: data.component_type_id,
        actual_status_id: data.actual_status_id,
        is_base: data.is_base,
    };

    let component: InsertableComponent = component_data.into();

    diesel::insert_into(component_ref::component_ref)
        .values(&component)
        .returning(component_ref::uuid)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed created standard: {:?}", err);
            ServiceError::InternalServerError
        })
}
