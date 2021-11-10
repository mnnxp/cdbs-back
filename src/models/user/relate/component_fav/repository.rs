use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::component::model::ShowComponentShort;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowComponentShort {
    /// get list subscribers for component
    pub(crate) fn _get_by_user_uuid(
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        let target_components_uuids = component_fav::component_fav
            .filter(component_fav::user_uuid.eq(target_user_uuid)
            .and(component_fav::is_enabled.eq(true)))
            .select(component_fav::component_uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowComponentShort::get_list_by_uuids(
            &target_components_uuids,
            target_user_uuid,
            set_lang_id,
            conn,
        )
    }
}

impl ComponentFav {
    /// Count subscribers for component
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(component_fav::component_fav
            .filter(component_fav::user_uuid.eq(target_user_uuid)
            .and(component_fav::is_enabled.eq(true)))
            .execute(conn)? as i32)
    }
}
