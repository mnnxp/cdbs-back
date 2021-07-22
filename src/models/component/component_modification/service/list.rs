use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::component_modification::model::ShowComponentModification;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_component_modifications(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    let mut variant_selection: u8 = 0;
    if uuid_component_search > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_component_modification(context, limit, offset),
        1 => find_uuid_component_modification(context, uuid_component_search, limit, offset),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_component_modification(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    use crate::schema::component_modification_list::dsl::*;
    // use crate::schema::program_ref::dsl::*;
    // use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_modification_list
        // .inner_join(program_ref)
        // .inner_join(actual_status_ref)
        .select((
            uuid, uuid_component, uuid_modification_parent, modification_name, description,
            id_actual_status, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponentModification>(conn)?)
}

fn find_uuid_component_modification(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    use crate::schema::component_modification_list::dsl::*;
    // use crate::schema::program_ref::dsl::*;
    // use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_modification_list
        // .inner_join(program_ref)
        // .inner_join(actual_status_ref)
        .filter(uuid_component.eq(uuid_component_search))
        .select((
            uuid, uuid_component, uuid_modification_parent, modification_name, description,
            id_actual_status, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponentModification>(conn)?)
}
