use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::models::user::standard_fav::model::{
    IptStandardFavData, InsertableStandardFav
};
use crate::schema::standard_fav::dsl as standard_fav;
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
    let check_fav = standard_fav::standard_fav
        .filter(standard_fav::standard_uuid.eq(&data.standard_uuid)
        .and(standard_fav::user_uuid.eq(&data.user_uuid)))
        .select(standard_fav::is_enabled)
        .first(conn);

    match check_fav {
        Ok(fav) => {
            if fav {
                // if data already has
                Ok(false)
            } else {
                // if have need row, just update is_enabled to true
                diesel::update(standard_fav::standard_fav)
                    .filter(standard_fav::standard_uuid.eq(&data.standard_uuid)
                    .and(standard_fav::user_uuid.eq(&data.user_uuid)))
                    .set(standard_fav::is_enabled.eq(true))
                    .returning(standard_fav::is_enabled)
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed add fav standard: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        },
        Err(err) => {
            debug!("Err check is_enabled: {:?}", err);

            // add flag and date created
            let insertable_fav: InsertableStandardFav = data.into();

            diesel::insert_into(standard_fav::standard_fav)
                .values(insertable_fav)
                .returning(standard_fav::is_enabled)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed add fav standard: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
