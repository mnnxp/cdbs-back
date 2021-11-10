use crate::errors::ServiceResult;
use super::model::RepresentationTypeTranslateList;
use crate::schema::representation_type_translate_list::dsl::*;
use diesel::prelude::*;

impl RepresentationTypeTranslateList {
    /// Get represent type with translate by ids
    pub fn get_by_ids(
        target_ids: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        Ok(representation_type_translate_list
            .filter(representation_type_id.eq_any(target_ids)
            .and(lang_id.eq(set_lang_id)))
            .load::<RepresentationTypeTranslateList>(conn)?)
    }
}
