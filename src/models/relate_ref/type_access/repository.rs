use crate::errors::ServiceResult;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::type_access_translate_list::dsl as type_access_translate_list;
use diesel::prelude::*;

impl TypeAccessTranslateList {
    pub fn get_type_access_by_id(
        target_type_access_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<TypeAccessTranslateList> {
        let type_access = type_access_translate_list::type_access_translate_list
            .filter(type_access_translate_list::type_access_id.eq(target_type_access_id)
            .and(type_access_translate_list::lang_id.eq(set_lang_id)))
            .first::<TypeAccessTranslateList>(conn);

        // if not found data for set lang
        match type_access {
            Ok(rn) => Ok(rn),
            Err(err) => {
                debug!("Not found set lang for type_access: {:?}", err);
                Ok(type_access_translate_list::type_access_translate_list
                    .filter(type_access_translate_list::type_access_id.eq(target_type_access_id))
                    .first::<TypeAccessTranslateList>(conn)?)
            },
        }
    }

    pub fn get_type_access_by_vec_id(
        target_vec_type_access_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        let type_accesss = type_access_translate_list::type_access_translate_list
            .filter(type_access_translate_list::type_access_id.eq_any(target_vec_type_access_id)
            .and(type_access_translate_list::lang_id.eq(set_lang_id)))
            .load::<TypeAccessTranslateList>(conn);

        // if not found data for set lang
        match type_accesss {
            Ok(rns) => Ok(rns),
            Err(err) => {
                debug!("Not found set lang for type_accesss: {:?}", err);
                Ok(type_access_translate_list::type_access_translate_list
                    .filter(type_access_translate_list::type_access_id.eq_any(target_vec_type_access_id))
                    .load::<TypeAccessTranslateList>(conn)?)
            },
        }
    }
}
