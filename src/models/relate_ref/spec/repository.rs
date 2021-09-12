use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    // Spec,
    SpecTranslateList,
};
use crate::schema::spec_translate_list::dsl as spec_translate_list;
use diesel::prelude::*;

// let spec_translate_list: Vec<SpecTranslateList> = spec_translate_list::spec_translate_list
//     .filter(spec_translate_list::spec_id.eq_any(spec_ids_for_component)
//     .and(spec_translate_list::lang_id.eq(set_lang_id)))
//     .load::<SpecTranslateList>(conn)
    // .expect("Error loading spec_translate_list");


impl SpecTranslateList {
    pub fn get_spec_by_id(
        target_spec_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<SpecTranslateList> {
        Ok(spec_translate_list::spec_translate_list
            .filter(spec_translate_list::spec_id.eq(target_spec_id)
            .and(spec_translate_list::lang_id.eq(set_lang_id)))
            .first::<SpecTranslateList>(conn)?)
    }

    pub fn get_spec_by_vec_id(
        target_vec_spec_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        Ok(spec_translate_list::spec_translate_list
            .filter(spec_translate_list::spec_id.eq_any(target_vec_spec_id)
            .and(spec_translate_list::lang_id.eq(set_lang_id)))
            .load::<SpecTranslateList>(conn)?)
    }
}
