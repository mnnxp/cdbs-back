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
    let check_fav = diesel::update(component_fav)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite component
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true), // <-- turned flag to false
        0_usize => {
            // component not found in favorite list
            Err(ServiceError::BadRequest(
                "No data found".to_string()
            ))
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
