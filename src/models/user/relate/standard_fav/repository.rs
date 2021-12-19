use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::standard_fav::model::StandardFav;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl StandardFav {
    /// Count subscribers for standard
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        let count = standard_fav::standard_fav
            .filter(standard_fav::user_uuid.eq(target_user_uuid)
            .and(standard_fav::is_enabled.eq(true)))
            .execute(conn)
            .map_err(|err| {
                debug!("Fail count company_fav: {:?} ", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
