use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_fav::model::ComponentFav;
use crate::schema::component_fav::dsl as component_fav;
use crate::subscribers_count;
use diesel::prelude::*;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref CHEAT_COMPONENT_UUID : Uuid =
        Uuid::parse_str("5c871507-1c44-43d7-986a-0934b90d850a")
            .expect("Set component uuid for cheat failed!");
}

/// Retund cheat component uuid
pub(crate) fn get_cheat() -> Uuid {
    *CHEAT_COMPONENT_UUID
}

impl ComponentFav {
    /// Count subscribers for component
    pub(crate) fn get_count_followers_by_uuid(
        target_component_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        // this component is added to the bookmarks of each new user
        if target_component_uuid == &get_cheat() {
            return Ok(10);
        }
        subscribers_count!(component_fav, component_uuid, target_component_uuid, conn)
    }
}
