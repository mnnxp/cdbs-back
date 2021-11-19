use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::standard_fav::model::IptStandardFavData;
use crate::schema::standard_fav::dsl::*;
use diesel::prelude::*;

// Remove a standard from user favorites standard list
pub(crate) fn delete_standard_fav(
    data: &IptStandardFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(standard_fav)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite standard
        .returning(is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
