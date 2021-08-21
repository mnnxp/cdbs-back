use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;


pub(crate) fn get_params(
    context: &Context<'_>,
    id_param_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match id_param_search {
        id_param_search if id_param_search.is_empty() => find_all_params(context, limit, offset),
        id_param_search => find_id_param(context, id_param_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_params(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(param_translate_list
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}

fn find_id_param(
    context: &Context<'_>,
    id_param_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(param_translate_list
        .filter(id_param.eq_any(id_param_search))
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}
