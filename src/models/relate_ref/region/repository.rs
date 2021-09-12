use crate::errors::ServiceResult;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::prelude::*;

impl RegionTranslateList {
    pub fn get_region_by_id(
        target_region_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<RegionTranslateList> {
        Ok(region_translate_list::region_translate_list
            .filter(region_translate_list::region_id.eq(target_region_id)
            .and(region_translate_list::lang_id.eq(set_lang_id)))
            .first::<RegionTranslateList>(conn)?)
    }

    pub fn get_region_by_vec_id(
        target_vec_region_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        Ok(region_translate_list::region_translate_list
            .filter(region_translate_list::region_id.eq_any(target_vec_region_id)
            .and(region_translate_list::lang_id.eq(set_lang_id)))
            .load::<RegionTranslateList>(conn)?)
    }
}
