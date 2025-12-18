use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_fav::model::ComponentFav;
use crate::schema::component_fav::dsl as component_fav;
use crate::subscribers_count;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// Count subscribers for component
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        subscribers_count!(component_fav, user_uuid, target_user_uuid, conn)
    }
}
