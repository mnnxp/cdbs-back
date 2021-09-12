use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::region::model::RegionTranslateList;
use diesel::prelude::*;


pub(crate) fn get_regions(
    cxt: &Context<'_>,
    region_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match region_id_search {
        region_id_search if region_id_search.is_empty() => find_all_regions(cxt, limit, offset),
        region_id_search => find_region_id(cxt, region_id_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_regions(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(region_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}

fn find_region_id(
    cxt: &Context<'_>,
    region_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(region_translate_list
        .filter(region_id.eq_any(region_id_search))
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}
