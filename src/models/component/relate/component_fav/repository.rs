use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// get list subscribers for component
    pub fn get_list_followers_by_uuid(
        target_uuid_component: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let target_list_uuid_user = component_fav::component_fav
            .filter(component_fav::uuid_component.eq(target_uuid_component))
            .select(component_fav::uuid_user)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowUserShort::get_list_by_uuids(&target_list_uuid_user, conn)
    }

    /// Count subscribers for component
    pub fn get_count_followers_by_uuid(
        target_uuid_component: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(component_fav::component_fav
            .filter(component_fav::uuid_component.eq(target_uuid_component))
            .execute(conn)? as i32)
    }
}
