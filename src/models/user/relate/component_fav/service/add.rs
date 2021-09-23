use crate::errors::ServiceResult;
use crate::models::user::component_fav::model::{
    ComponentFav,
    IptComponentFavData,
    InsertableComponentFav,
};
use crate::schema::component_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_component_fav(
    data: IptComponentFavData,
    conn: &PgConnection,
) -> ServiceResult<ComponentFav> {
    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(component_fav)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .get_result(conn);

    let user_component_fav: ComponentFav = match check_fav {
        Ok(fav) => fav,
        Err(_) => {
            // add flag and date created
            let insertable_fav: InsertableComponentFav = data.into();

            diesel::insert_into(component_fav)
                .values(insertable_fav)
                .get_result(conn)?
        },
    };

    debug!("User favorite component: {:#?}", user_component_fav);

    Ok(user_component_fav)
}
