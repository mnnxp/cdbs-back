use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::schema::component_type_translate_list::dsl as component_type_translate_list;
use diesel::prelude::*;

impl ComponentTypeTranslateList {
    /// Get component type by id
    pub(crate) fn get_by_id(
        component_type_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<ComponentTypeTranslateList> {
        let component_type = component_type_translate_list::component_type_translate_list
            .filter(component_type_translate_list::component_type_id.eq(component_type_id)
            .and(component_type_translate_list::lang_id.eq(set_lang_id)))
            .limit(1)
            .load::<ComponentTypeTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get component type: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match component_type.first() {
            Some(x) => Ok(x.clone()),
            None => {
                debug!("Not found set lang for component type");
                component_type_translate_list::component_type_translate_list
                    .filter(component_type_translate_list::component_type_id.eq(component_type_id))
                    .first::<ComponentTypeTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get component type: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }

    /// Get component type by ids and set lang
    /// if filter empty return all statuses
    pub(crate) fn get_by_ids(
        filter: &[i32],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ComponentTypeTranslateList>> {
        let res = match filter.is_empty() {
            true => component_type_translate_list::component_type_translate_list
                .filter(component_type_translate_list::lang_id.eq(set_lang_id))
                .load::<ComponentTypeTranslateList>(conn),
            false => component_type_translate_list::component_type_translate_list
                .filter(component_type_translate_list::component_type_id.eq_any(filter)
                .and(component_type_translate_list::lang_id.eq(set_lang_id)))
                .load::<ComponentTypeTranslateList>(conn),
        };

        res.map_err(|err| {
            debug!("Failed get component type: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
