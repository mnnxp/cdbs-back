use crate::database::PooledConnection;
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::graphql::model::Context;
use crate::models::component_modification::model::ComponentModification;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentModification>> {
    // debug!("fn show uuid_component_search = {}", &uuid_component_search);
    let uuid_component_search = Some(uuid_component_search);
    match uuid_component_search {
        Some(uuid_component_search) if uuid_component_search == Uuid::nil() => find_all_component_modification(context, limit, offset),
        Some(uuid_component_search) if uuid_component_search > Uuid::nil() => find_uuid_component_modification(context, uuid_component_search, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_component_modification(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentModification>> {
    use crate::schema::component_modification_list::dsl::component_modification_list;
    let conn: &PooledConnection = &context.db;

    Ok(component_modification_list
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentModification>(conn)?)
}

fn find_uuid_component_modification(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentModification>> {
    use crate::schema::component_modification_list::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_modification_list
        .filter(uuid_component.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentModification>(conn)?)
}
