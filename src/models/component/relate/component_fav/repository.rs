use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
// use crate::models::user::model::ShowUserShort;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// Count subscribers for component
    pub(crate) fn get_count_followers_by_uuid(
        target_component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(component_fav::component_fav
            .filter(component_fav::component_uuid.eq(target_component_uuid))
            .execute(conn)? as i32)
    }
}
