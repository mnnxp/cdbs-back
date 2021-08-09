use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::component_modification::param::model::ParamModification;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_params_modification(
    context: &Context<'_>,
    id_param_search: i32,
    uuid_modification_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamModification>> {
    let mut variant_selection: u8 = 0;
    if id_param_search > 0 {
        variant_selection += 1;
    }
    if uuid_modification_search > Uuid::nil() {
        variant_selection += 10;
    }

    match variant_selection {
        0 => find_all_params(context, limit, offset),
        1 => find_id_param(context, id_param_search, limit, offset),
        10 => find_uuid_modification_param(context, uuid_modification_search, limit, offset),
        11 => find_id_param_and_uuid_modification(context, id_param_search, uuid_modification_search, limit, offset),
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
) -> ServiceResult<Vec<ParamModification>> {
    use crate::schema::param_to_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(param_to_modification
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamModification>(conn)?)
}

fn find_id_param(
    context: &Context<'_>,
    id_param_search: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamModification>> {
    use crate::schema::param_to_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(param_to_modification
        .filter(id_param.eq(id_param_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamModification>(conn)?)
}

fn find_uuid_modification_param(
    context: &Context<'_>,
    uuid_modification_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamModification>> {
    use crate::schema::param_to_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(param_to_modification
        .filter(uuid_modification.eq(uuid_modification_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamModification>(conn)?)
}

fn find_id_param_and_uuid_modification(
    context: &Context<'_>,
    id_param_search: i32,
    uuid_modification_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamModification>> {
    use crate::schema::param_to_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(param_to_modification
        .filter(uuid_modification.eq(uuid_modification_search))
        .filter(id_param.eq(id_param_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamModification>(conn)?)
}
