use crate::errors::ServiceResult;
use crate::models::relate_ref::region::model::RegionTranslateList;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_regions(
    region_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match region_id_search {
        region_id_search if region_id_search.is_empty() => find_all_regions(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        region_id_search => find_region_id(
            region_id_search,
            limit,
            offset,
            set_lang_id,
            conn,
        )
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_regions(
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;

    Ok(region_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}

fn find_region_id(
    region_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    use crate::schema::region_translate_list::dsl::*;

    Ok(region_translate_list
        .filter(region_id.eq_any(region_id_search)
        .and(lang_id.eq(set_lang_id)))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<RegionTranslateList>(conn)?)
}
