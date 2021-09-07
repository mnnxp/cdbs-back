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
    context: &Context<'_>,
    id_region_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match id_region_search {
        id_region_search if id_region_search.is_empty() => find_all_regions(context, limit, offset),
        id_region_search => find_id_region(context, id_region_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_regions(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(region_translate_list
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}

fn find_id_region(
    context: &Context<'_>,
    id_region_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(region_translate_list
        .filter(id_region.eq_any(id_region_search))
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}
