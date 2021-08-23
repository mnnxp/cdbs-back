use crate::errors::ServiceResult;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::schema::component_type_translate_list::dsl as component_type_translate_list;
use diesel::prelude::*;

impl ComponentTypeTranslateList {
    pub fn get_component_type_by_id(
        target_id_component_type: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentTypeTranslateList> {
        Ok(component_type_translate_list::component_type_translate_list
            .filter(component_type_translate_list::id_component_type.eq(target_id_component_type)
            .and(component_type_translate_list::id_lang.eq(set_id_lang)))
            .first::<ComponentTypeTranslateList>(conn)?)
    }
}
