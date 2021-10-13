use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::company_fav::model::IptCompanyFavData;
use crate::schema::company_fav::dsl::*;
use diesel::prelude::*;

// Remove a company from user favorites company list
pub(crate) fn delete_company_fav(
    data: &IptCompanyFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let check_fav = diesel::update(company_fav)
        .filter(company_uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite company
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true), // <-- turned flag to false
        0_usize => {
            // company not found in favorite list
            Err(ServiceError::BadRequest(
                "No data found".to_string()
            ))
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
