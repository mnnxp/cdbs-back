use crate::errors::ServiceResult;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::prelude::*;

impl RegionTranslateList {
    pub fn get_region_by_id(
        target_id_region: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<RegionTranslateList> {
        Ok(region_translate_list::region_translate_list
            .filter(region_translate_list::id_region.eq(target_id_region)
            .and(region_translate_list::id_lang.eq(set_id_lang)))
            .first::<RegionTranslateList>(conn)?)
    }

    pub fn get_region_by_vec_id(
        target_vec_id_region: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        Ok(region_translate_list::region_translate_list
            .filter(region_translate_list::id_region.eq_any(target_vec_id_region)
            .and(region_translate_list::id_lang.eq(set_id_lang)))
            .load::<RegionTranslateList>(conn)?)
    }
}
