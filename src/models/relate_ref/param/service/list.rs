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
    cxt: &Context<'_>,
    param_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match param_id_search {
        param_id_search if param_id_search.is_empty() => find_all_params(cxt, limit, offset),
        param_id_search => find_param_id(cxt, param_id_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_params(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(param_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}

fn find_param_id(
    cxt: &Context<'_>,
    param_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(param_translate_list
        .filter(param_id.eq_any(param_id_search))
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}
