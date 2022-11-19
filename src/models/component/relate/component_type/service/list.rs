use crate::errors::ServiceResult;
use crate::models::component::relate::component_type::model::ComponentTypeTranslateList;
use diesel::PgConnection;

/// Gets component types list
pub(crate) fn get_component_types(
    filter: &[i32],
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ComponentTypeTranslateList>> {
    ComponentTypeTranslateList::get_by_ids(filter, set_lang_id, conn)
}
