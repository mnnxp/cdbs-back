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
    let check_fav = diesel::update(standard_fav)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite standard
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true), // <-- turned flag to false
        0_usize => {
            // standard not found in favorite list
            Err(ServiceError::BadRequest(
                "No data found".to_string()
            ))
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
