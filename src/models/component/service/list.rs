use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::component::model::Component;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    match uuid_component_search {
        uuid_component_search if uuid_component_search == Uuid::nil() => find_all_components(context, limit, offset),
        uuid_component_search if uuid_component_search > Uuid::nil() => find_uuid_component(context, uuid_component_search, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_components(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    use crate::schema::component_ref::dsl::component_ref;
    let conn: &PooledConnection = &context.db;

    Ok(component_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Component>(conn)?)
}

fn find_uuid_component(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    use crate::schema::component_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_ref
        .filter(uuid.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Component>(conn)?)
}
