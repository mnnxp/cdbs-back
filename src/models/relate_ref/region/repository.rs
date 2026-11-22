use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::search::order::Paginate;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::prelude::*;
use diesel::PgConnection;

impl RegionTranslateList {
    pub(crate) fn find_all_regions(
        set_lang_id: i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        region_translate_list::region_translate_list
            .filter(region_translate_list::lang_id.eq(set_lang_id))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .order(region_translate_list::region.asc())
            .load::<RegionTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get region: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    pub(crate) fn find_region_ids(
        region_ids: &[i32],
        set_lang_id: i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        region_translate_list::region_translate_list
            .filter(
                region_translate_list::region_id
                    .eq_any(region_ids)
                    .and(region_translate_list::lang_id.eq(set_lang_id)),
            )
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<RegionTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get region: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
