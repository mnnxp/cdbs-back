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

    let parent_component_uuid = match &data.parent_component_uuid {
        Some(parent) => Uuid::parse_str(parent)?,
        None => Uuid::parse_str("a5953fd9-7393-4f1e-a899-06b5e159dbf1")?, // <-- todo!(get uuid root component)
    };

    let component_data = ComponentData {
        parent_component_uuid: (parent_component_uuid),
        name: (data.name),
        description: (data.description),
        user_uuid: (logged_user_uuid),
        type_access_id: (data.type_access_id),
        component_type_id: (data.component_type_id),
        actual_status_id: (data.actual_status_id),
        is_standard: (data.is_standard),
    };

    let component: InsertableComponent = component_data.into();
    let inserted_component: Component = diesel::insert_into(component_ref).values(&component).get_result(conn)?;
    Ok(inserted_component.into())
}
