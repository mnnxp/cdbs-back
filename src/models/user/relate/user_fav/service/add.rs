use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::user::user_fav::model::{
    UserFav,
    IptUserFavData,
    InsertableUserFav
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_user_favorite(
    data: IptUserFavData,
    conn: &PgConnection
) -> ServiceResult<UserFav> {
    use crate::schema::user_fav::dsl::*;

    let new_user_favorite: InsertableUserFav = data.into();

    let flag_found_favorite = user_fav
        .filter(user_uuid.eq(&new_user_favorite.user_uuid)
        .and(user_uuid.eq(&new_user_favorite.user_uuid)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    match flag_found_favorite as i32 {
        0 => {
            let inserted_user_favorite: UserFav = diesel::insert_into(user_fav)
                .values(&new_user_favorite)
                .get_result(conn)?;
            Ok(inserted_user_favorite)
        },
        _ => Err(ServiceError::BadRequest("This favorite name is already with the user.".to_string())),
    }
}
