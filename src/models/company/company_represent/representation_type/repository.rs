use crate::errors::ServiceResult;
use super::model::RepresentationTypeTranslateList;
use crate::schema::representation_type_translate_list::dsl::*;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get company typeanization by id and set lang
    pub fn get_representation_type_by_id(
        target_representation_type_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<RepresentationTypeTranslateList> {
        Ok(representation_type_translate_list
            .filter(representation_type_id.eq(target_representation_type_id)
            .and(lang_id.eq(set_lang_id)))
            .first::<RepresentationTypeTranslateList>(conn)?)
    }

    /// Get list company typeanization by vec id and set lang
    pub fn get_representation_type_by_vec_id(
        target_vec_representation_type_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        Ok(representation_type_translate_list
            .filter(representation_type_id.eq_any(target_vec_representation_type_id)
            .and(lang_id.eq(set_lang_id)))
            .load::<RepresentationTypeTranslateList>(conn)?)
    }
}
