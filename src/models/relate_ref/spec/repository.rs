use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    // Spec,
    SpecTranslateList,
};
use crate::schema::spec_translate_list::dsl as spec_translate_list;
use diesel::prelude::*;

// let spec_translate_list: Vec<SpecTranslateList> = spec_translate_list::spec_translate_list
//     .filter(spec_translate_list::id_spec.eq_any(id_specs_for_component)
//     .and(spec_translate_list::id_lang.eq(set_id_lang)))
//     .load::<SpecTranslateList>(conn)
    // .expect("Error loading spec_translate_list");


impl SpecTranslateList {
    pub fn get_spec_by_id(
        target_id_spec: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<SpecTranslateList> {
        Ok(spec_translate_list::spec_translate_list
            .filter(spec_translate_list::id_spec.eq(target_id_spec)
            .and(spec_translate_list::id_lang.eq(set_id_lang)))
            .first::<SpecTranslateList>(conn)?)
    }

    pub fn get_spec_by_vec_id(
        target_vec_id_spec: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        Ok(spec_translate_list::spec_translate_list
            .filter(spec_translate_list::id_spec.eq_any(target_vec_id_spec)
            .and(spec_translate_list::id_lang.eq(set_id_lang)))
            .load::<SpecTranslateList>(conn)?)
    }
}
