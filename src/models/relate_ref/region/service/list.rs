use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::region::model::{
    RegionTranslateList, RegionArg
};
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_regions(
    args: &RegionArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match args.region_ids.is_empty() {
        true => find_all_regions(&args.limit, &args.offset, set_lang_id, conn),
        false => find_region_id(args, set_lang_id, conn),
    }
}

fn find_all_regions(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    region_translate_list::region_translate_list
        .filter(region_translate_list::lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .order(region_translate_list::region.asc())
        .load::<RegionTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get region: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_region_id(
    args: &RegionArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    region_translate_list::region_translate_list
        .filter(region_translate_list::region_id.eq_any(&args.region_ids)
        .and(region_translate_list::lang_id.eq(set_lang_id)))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<RegionTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get region: {:?}", err);
            ServiceError::InternalServerError
        })
}
