use crate::errors::ServiceResult;
use super::model::RepresentationTypeTranslateList;
use crate::schema::representation_type_translate_list::dsl::*;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get company typeanization by id and set lang
    pub fn get_representation_type_by_id(
        target_id_representation_type: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<RepresentationTypeTranslateList> {
        Ok(representation_type_translate_list
            .filter(id_representation_type.eq(target_id_representation_type)
            .and(id_lang.eq(set_id_lang)))
            .first::<RepresentationTypeTranslateList>(conn)?)
    }

    /// Get list company typeanization by vec id and set lang
    pub fn get_representation_type_by_vec_id(
        target_vec_id_representation_type: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        Ok(representation_type_translate_list
            .filter(id_representation_type.eq_any(target_vec_id_representation_type)
            .and(id_lang.eq(set_id_lang)))
            .load::<RepresentationTypeTranslateList>(conn)?)
    }
}
