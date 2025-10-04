use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::standard_fav::model::StandardFav;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl StandardFav {
    /// Count subscribers for standard
    pub(crate) fn get_count_followers_by_uuid(
        target_standard_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        let count = standard_fav::standard_fav
            .filter(standard_fav::standard_uuid.eq(target_standard_uuid))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed get actual status: {:?}", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
