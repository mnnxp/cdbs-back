use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::user::component_fav::model::{
    IptComponentFavData, InsertableComponentFav
};
use crate::schema::component_fav::dsl as component_fav;
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
    let check_fav = component_fav::component_fav
        .filter(component_fav::component_uuid.eq(&data.component_uuid)
        .and(component_fav::user_uuid.eq(&data.user_uuid)))
        .select(component_fav::is_enabled)
        .first(conn);

    match check_fav {
        Ok(fav) => {
            if fav {
                // if data already has
                Ok(false)
            } else {
                // if have need row, just update is_enabled to true
                diesel::update(component_fav::component_fav)
                    .filter(component_fav::component_uuid.eq(&data.component_uuid)
                    .and(component_fav::user_uuid.eq(&data.user_uuid)))
                    .set(component_fav::is_enabled.eq(true))
                    .returning(component_fav::is_enabled)
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed add fav component: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        },
        Err(err) => {
            debug!("Err check is_enabled: {:?}", err);

            // add flag and date created
            let insertable_fav: InsertableComponentFav = data.into();

            diesel::insert_into(component_fav::component_fav)
                .values(insertable_fav)
                .returning(component_fav::is_enabled)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed add fav component: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
