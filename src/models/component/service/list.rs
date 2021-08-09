use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceResult, ServiceError};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::model::ShowComponent;
use crate::models::component::model::ShowComponentFull;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_components(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    let mut variant_selection: u8 = 0;
    if target_uuid_component > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_components(context, limit, offset),
        1 => find_uuid_component(context, target_uuid_component, limit, offset),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_components(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    use crate::schema::component_ref::dsl::*;
    // use crate::schema::actual_status_ref::dsl::*;
    // use crate::schema::component_type_ref::dsl::*;
    // use crate::schema::type_access_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_ref
        // .inner_join(actual_status_ref)
        // .inner_join(component_type_ref)
        // .inner_join(type_access_ref)
        .select((
            uuid, uuid_component_parent, name, description, uuid_user,
            id_type_access, id_component_type, id_actual_status,
            is_standard, is_delete, created_at, updated_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponent>(conn)?)
}

fn find_uuid_component(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    use crate::schema::component_ref::dsl::*;
    // use crate::schema::actual_status_ref::dsl::*;
    // use crate::schema::component_type_ref::dsl::*;
    // use crate::schema::type_access_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_ref
        // .inner_join(actual_status_ref)
        // .inner_join(component_type_ref)
        // .inner_join(type_access_ref)
        .filter(uuid.eq(target_uuid_component))
        .select((
            uuid, uuid_component_parent, name, description, uuid_user,
            id_type_access, id_component_type, id_actual_status,
            is_standard, is_delete, created_at, updated_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponent>(conn)?)
}

pub(crate) fn get_one_components(
    context: &Context<'_>,
    target_uuid_component: Uuid,
) -> ServiceResult<ShowComponentFull> {
    use crate::models::component::model::Component;
    use crate::models::component::param::model::ParamComponent;
    use crate::models::component::license::model::LicenseComponent;
    use crate::models::component::component_modification::model::ComponentModification;
    use crate::models::component::component_modification::param::model::ParamModification;
    use crate::schema::component_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let component = component_ref
        .filter(uuid.eq(target_uuid_component))
        .first::<Component>(conn)
        .expect("Error loading component");
    let param_component = ParamComponent::belonging_to(&component)
        .load::<ParamComponent>(conn)
        .expect("Error loading param_component");
    let license = LicenseComponent::belonging_to(&component)
        .load::<LicenseComponent>(conn)
        .expect("Error loading license");
    // let files: Vec<ShowFile> = ShowFile.grouped_by(&posts);
    let component_modification = ComponentModification::belonging_to(&component)
        .load::<ComponentModification>(conn)
        .expect("Error loading component_modification");
    let param_modification = ParamModification::belonging_to(&component_modification)
        .load::<ParamModification>(conn)
        .expect("Error loading param_modification");
    let result = ShowComponentFull {
        component,
        param_component,
        license,
        // files,
        component_modification,
        param_modification 
    };
    Ok(result)
}
