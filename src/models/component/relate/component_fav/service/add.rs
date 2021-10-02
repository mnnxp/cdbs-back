use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_fav::model::{
    ComponentFav,
    IptComponentFavData,
    InsertableComponentFav
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_component_favorite(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<ComponentFav> {
    use crate::schema::component_fav::dsl::*;

    let data = IptComponentFavData {
        component_uuid: *target_component_uuid,
        user_uuid: *logged_user_uuid,
    };

    let new_component_favorite: InsertableComponentFav = data.into();

    let flag_found_favorite = component_fav
        .filter(component_uuid.eq(&new_component_favorite.component_uuid)
        .and(user_uuid.eq(&new_component_favorite.user_uuid)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    match flag_found_favorite as i32 {
        0 => {
            let inserted_component_favorite: ComponentFav = diesel::insert_into(component_fav)
                .values(&new_component_favorite)
                .get_result(conn)?;
            Ok(inserted_component_favorite)
        },
        _ => Err(ServiceError::BadRequest("This favorite name is already with the component.".to_string())),
    }
}
