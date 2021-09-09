use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::component::model::ShowComponentShort;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// get list subscribers for component
    pub fn get_by_user_uuid(
        target_user_uuid: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        let target_components_uuids = component_fav::component_fav
            .filter(component_fav::uuid_user.eq(target_user_uuid))
            .select(component_fav::uuid_component)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowComponentShort::get_list_by_uuids(
            &target_components_uuids,
            target_user_uuid,
            set_id_lang,
            conn,
        )
    }

    /// Count subscribers for component
    pub fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(component_fav::component_fav
            .filter(component_fav::uuid_user.eq(target_user_uuid))
            .execute(conn)? as i32)
    }
}
