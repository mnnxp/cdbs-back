use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::param::model::ParamToModel;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_params_component(
    context: &Context<'_>,
    id_param_search: i32,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamToModel>> {
    let mut variant_selection: u8 = 0;
    if id_param_search > 0 {
        variant_selection += 1;
    }
    if uuid_component_search > Uuid::nil() {
        variant_selection += 10;
    }

    match variant_selection {
        0 => find_all_params(context, limit, offset),
        1 => find_id_param(context, id_param_search, limit, offset),
        10 => find_uuid_component_param(context, uuid_component_search, limit, offset),
        11 => find_id_param_and_uuid_component(context, id_param_search, uuid_component_search, limit, offset),
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_params(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamToModel>> {
    use crate::schema::param_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(param_to_component
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamToModel>(conn)?)
}

fn find_id_param(
    context: &Context<'_>,
    id_param_search: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamToModel>> {
    use crate::schema::param_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(param_to_component
        .filter(id_param.eq(id_param_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamToModel>(conn)?)
}

fn find_uuid_component_param(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamToModel>> {
    use crate::schema::param_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(param_to_component
        .filter(uuid_component.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamToModel>(conn)?)
}

fn find_id_param_and_uuid_component(
    context: &Context<'_>,
    id_param_search: i32,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamToModel>> {
    use crate::schema::param_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(param_to_component
        .filter(uuid_component.eq(uuid_component_search))
        .filter(id_param.eq(id_param_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamToModel>(conn)?)
}
