use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::user::standard_fav::model::{
    StandardFav,
    IptStandardFavData,
};
use crate::schema::standard_fav::dsl::*;
use diesel::prelude::*;

// Remove a standard from user favorites standard list
pub(crate) fn delete_standard_fav(
    data: &IptStandardFavData,
    conn: &PgConnection,
) -> ServiceResult<StandardFav> {
    // if have need row, just update is_enabled to false
    let check_fav = diesel::update(standard_fav)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite standard
        .get_result(conn);

    let user_standard_fav: StandardFav = match check_fav {
        Ok(fav) => fav, // <-- turned flag to false
        Err(err) => {
            debug!("Err with delete standard fav: {:#?}", err);
            // standard not found in favorite list
            return Err(ServiceError::BadRequest("Standard not found in favotite list".to_string()))
        },
    };

    debug!("Standard delete from favorute: {:#?}", user_standard_fav);

    Ok(user_standard_fav)
}
