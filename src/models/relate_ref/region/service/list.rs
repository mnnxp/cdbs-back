use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::region_translate_list::dsl::*;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_regions(
    region_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match region_id_search.is_empty() {
        true => find_all_regions(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        false => find_region_id(
            region_id_search,
            limit,
            offset,
            set_lang_id,
            conn,
        )
    }
}

fn find_all_regions(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    region_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<RegionTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get region: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_region_id(
    region_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    region_translate_list
        .filter(region_id.eq_any(region_id_search)
        .and(lang_id.eq(set_lang_id)))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<RegionTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get region: {:?}", err);
            ServiceError::InternalServerError
        })
}
