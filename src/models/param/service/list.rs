use crate::database::PooledConnection;
use crate::errors::{
    // ServiceError,
    ServiceResult
};
use crate::graphql::model::Context;
use crate::models::param::model::Param;
use diesel::prelude::*;


pub(crate) fn show(
    context: &Context,
    id_param_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Param>> {
    match id_param_search {
        id_param_search if id_param_search.is_empty() => find_all_param(context, limit, offset),
        id_param_search => find_id_param(context, id_param_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_param(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Param>> {
    use crate::schema::param_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(param_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Param>(conn)?)
}

fn find_id_param(
    context: &Context,
    id_param_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Param>> {
    use crate::schema::param_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(param_ref
        .filter(id.eq_any(id_param_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Param>(conn)?)
}
