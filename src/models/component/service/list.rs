use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceResult, ServiceError};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::model::ShowComponent;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_components(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    let mut variant_selection: u8 = 0;
    if uuid_component_search > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_components(context, limit, offset),
        1 => find_uuid_component(context, uuid_component_search, limit, offset),
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
    let conn: &PooledConnection = &get_conn(&context)?;

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
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    use crate::schema::component_ref::dsl::*;
    // use crate::schema::actual_status_ref::dsl::*;
    // use crate::schema::component_type_ref::dsl::*;
    // use crate::schema::type_access_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(component_ref
        // .inner_join(actual_status_ref)
        // .inner_join(component_type_ref)
        // .inner_join(type_access_ref)
        .filter(uuid.eq(uuid_component_search))
        .select((
            uuid, uuid_component_parent, name, description, uuid_user,
            id_type_access, id_component_type, id_actual_status,
            is_standard, is_delete, created_at, updated_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponent>(conn)?)
}
