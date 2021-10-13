use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::models::user::standard_fav::model::{
    IptStandardFavData, InsertableStandardFav
};
use crate::schema::standard_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_standard_fav(
    data: &IptStandardFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for standard
    check_access_standard_for_user(
        &data.user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(standard_fav)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true),
        0_usize => {
            // add flag and date created
            let insertable_fav: InsertableStandardFav = data.into();

            diesel::insert_into(standard_fav)
                .values(insertable_fav)
                .execute(conn)
                .expect("Failed add fav data");

            Ok(true)
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
