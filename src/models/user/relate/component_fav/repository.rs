use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_fav::model::ComponentFav;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentFav {
    /// Count subscribers for component
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        let count = component_fav::component_fav
            .filter(component_fav::user_uuid.eq(target_user_uuid)
            .and(component_fav::is_enabled.eq(true)))
            .execute(conn)
            .map_err(|err| {
                debug!("Fail count company_fav: {:?} ", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
