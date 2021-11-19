use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::component_fav::model::IptComponentFavData;
use crate::schema::component_fav::dsl::*;
use diesel::prelude::*;

// Remove a component from user favorites component list
pub(crate) fn delete_component_fav(
    data: &IptComponentFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(component_fav)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite component
        .returning(is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
