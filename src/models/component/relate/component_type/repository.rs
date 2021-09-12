use crate::errors::ServiceResult;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::schema::component_type_translate_list::dsl as component_type_translate_list;
use diesel::prelude::*;

impl ComponentTypeTranslateList {
    /// Get component type by id
    pub fn get_component_type_by_id(
        target_component_type_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentTypeTranslateList> {
        Ok(component_type_translate_list::component_type_translate_list
            .filter(component_type_translate_list::component_type_id.eq(target_component_type_id)
            .and(component_type_translate_list::lang_id.eq(set_lang_id)))
            .first::<ComponentTypeTranslateList>(conn)?)
    }
}
