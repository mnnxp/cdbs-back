use crate::errors::ServiceResult;
use crate::models::component::model::{
    IptComponentData,
    InsertableComponent,
    SlimComponent,
    Component,
    ComponentData
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component(
    logged_user_uuid: Uuid,
    data: IptComponentData,
    conn: &PgConnection
) -> ServiceResult<SlimComponent> {
    use crate::schema::component_ref::dsl::component_ref;

    let uuid_component_parent = match &data.uuid_component_parent {
        Some(parent) => Uuid::parse_str(parent)?,
        None => Uuid::parse_str("a5953fd9-7393-4f1e-a899-06b5e159dbf1")?, // <-- todo!(get uuid root component)
    };

    let component_data = ComponentData {
        uuid_component_parent: (uuid_component_parent),
        name: (data.name),
        description: (data.description),
        uuid_user: (logged_user_uuid),
        id_type_access: (data.id_type_access),
        id_component_type: (data.id_component_type),
        id_actual_status: (data.id_actual_status),
        is_standard: (data.is_standard),
    };

    let component: InsertableComponent = component_data.into();
    let inserted_component: Component = diesel::insert_into(component_ref).values(&component).get_result(conn)?;
    Ok(inserted_component.into())
}
