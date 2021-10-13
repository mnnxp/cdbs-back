use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::user::component_fav::model::{
    IptComponentFavData, InsertableComponentFav
};
use crate::schema::component_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_component_fav(
    data: &IptComponentFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for component
    check_access_component_for_user(
        &data.user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(component_fav)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true),
        0_usize => {
            // add flag and date created
            let insertable_fav: InsertableComponentFav = data.into();

            diesel::insert_into(component_fav)
                .values(insertable_fav)
                .execute(conn)
                .expect("Failed add fav data");

            Ok(true)
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
