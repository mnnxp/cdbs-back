use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::component::model::ShowComponent;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
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
) -> ServiceResult<Vec<ShowComponent>> {
    use crate::schema::component_ref::dsl::*;
    use crate::schema::actual_status_ref::dsl::*;
    use crate::schema::component_type_ref::dsl::*;
    use crate::schema::type_access_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_ref
        .inner_join(actual_status_ref)
        .inner_join(component_type_ref)
        .inner_join(type_access_ref)
        .select((
            uuid, name, uuid_user, comment, uuid_component_parent,
            id_actual_status, actualstatus, id_component_type,
            component_type, is_delete, id_type_access,
            type_access, commentchange, is_standard, created_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponent>(conn)?)
}

fn find_uuid_component(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponent>> {
    use crate::schema::component_ref::dsl::*;
    use crate::schema::actual_status_ref::dsl::*;
    use crate::schema::component_type_ref::dsl::*;
    use crate::schema::type_access_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_ref
        .inner_join(actual_status_ref)
        .inner_join(component_type_ref)
        .inner_join(type_access_ref)
        .select((
            uuid, name, uuid_user, comment, uuid_component_parent,
            id_actual_status, actualstatus, id_component_type,
            component_type, is_delete, id_type_access,
            type_access, commentchange, is_standard, created_at
        ))
        .filter(uuid.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponent>(conn)?)
}
