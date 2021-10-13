use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::user::component_fav::model::{
    ComponentFav,
    IptComponentFavData,
};
use crate::schema::component_fav::dsl::*;
use diesel::prelude::*;

// Remove a component from user favorites component list
pub(crate) fn delete_component_fav(
    data: &IptComponentFavData,
    conn: &PgConnection,
) -> ServiceResult<ComponentFav> {
    // if have need row, just update is_enabled to false
    let check_fav = diesel::update(component_fav)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite component
        .get_result(conn);

    let user_component_fav: ComponentFav = match check_fav {
        Ok(fav) => fav, // <-- turned flag to false
        Err(err) => {
            debug!("Err with delete component fav: {:#?}", err);
            // component not found in favorite list
            return Err(ServiceError::BadRequest("Component not found in favotite list".to_string()))
        },
    };

    debug!("Component delete from favorute: {:#?}", user_component_fav);

    Ok(user_component_fav)
}
