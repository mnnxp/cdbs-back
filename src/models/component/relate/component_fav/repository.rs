use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// get list subscribers for component
    pub fn get_list_followers_by_uuid(
        target_component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let target_list_user_uuid = component_fav::component_fav
            .filter(component_fav::component_uuid.eq(target_component_uuid))
            .select(component_fav::user_uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowUserShort::get_list_by_uuids(&target_list_user_uuid, conn)
    }

    /// Count subscribers for component
    pub fn get_count_followers_by_uuid(
        target_component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(component_fav::component_fav
            .filter(component_fav::component_uuid.eq(target_component_uuid))
            .execute(conn)? as i32)
    }
}
