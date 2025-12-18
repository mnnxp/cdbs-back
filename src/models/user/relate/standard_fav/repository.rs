use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::standard_fav::model::StandardFav;
use crate::schema::standard_fav::dsl as standard_fav;
use crate::subscribers_count;
use diesel::prelude::*;
use uuid::Uuid;

impl StandardFav {
    /// Count subscribers for standard
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        subscribers_count!(standard_fav, user_uuid, target_user_uuid, conn)
    }
}
