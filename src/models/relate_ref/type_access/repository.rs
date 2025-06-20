use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::type_access_translate_list::dsl::*;
use diesel::prelude::*;

impl TypeAccessTranslateList {
    /// Get access type by id
    pub(crate) fn get_type_access_by_id(
        target_type_access_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<TypeAccessTranslateList> {
        let type_access = type_access_translate_list
            .filter(
                type_access_id
                    .eq(target_type_access_id)
                    .and(lang_id.eq(set_lang_id)),
            )
            .limit(1)
            .load::<TypeAccessTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get type access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // if not found data for set lang
        match type_access.first() {
            Some(x) => Ok(x.clone()),
            None => {
                debug!("Not found set lang for type_access");
                type_access_translate_list
                    .filter(type_access_id.eq(target_type_access_id))
                    .first::<TypeAccessTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get type access: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        }
    }

    /// Get access types by IDs
    pub(crate) fn get_types_access_by_ids(
        target_types_access_ids: &[i32],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        let type_access = type_access_translate_list
            .filter(
                type_access_id
                    .eq_any(target_types_access_ids)
                    .and(lang_id.eq(set_lang_id)),
            )
            .load::<TypeAccessTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get type access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // if not found data for set lang
        match type_access.is_empty() {
            true => {
                debug!("Not found set lang for type_access");
                type_access_translate_list
                    .filter(type_access_id.eq_any(target_types_access_ids))
                    .load::<TypeAccessTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get type access: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
            false => Ok(type_access),
        }
    }
}
