use crate::errors::{ServiceResult, ServiceError};
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the component
pub(crate) fn check_subscriber_by_uuid(
    target_component_uuid: &Uuid,
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = component_fav::component_fav
        .filter(component_fav::component_uuid.eq(target_component_uuid)
        .and(component_fav::user_uuid.eq(target_user_uuid))
        .and(component_fav::is_enabled.eq(true)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(matches!(check_subscriber, x if x > 0))
}
