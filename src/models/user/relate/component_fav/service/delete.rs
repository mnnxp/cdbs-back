use crate::errors::{ServiceError, ServiceResult};
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

// Remove a component from user favorites component list
pub(crate) fn delete_component_fav(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(component_fav::component_fav)
        .filter(component_fav::component_uuid.eq(component_uuid)
        .and(component_fav::user_uuid.eq(logged_user_uuid))
        .and(component_fav::is_enabled.eq(true))) // <-- active favorite
        .set(component_fav::is_enabled.eq(false)) // <-- off favorite component
        .returning(component_fav::is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
