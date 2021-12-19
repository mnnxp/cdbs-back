use crate::errors::{ServiceResult, ServiceError};
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
        let count = component_fav::component_fav
            .filter(component_fav::component_uuid.eq(target_component_uuid))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed get actual status: {:?}", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
