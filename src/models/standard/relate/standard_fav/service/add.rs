use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::standard::standard_fav::model::{
    StandardFav,
    IptStandardFavData,
    InsertableStandardFav
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_standard_favorite(
    data: IptStandardFavData,
    conn: &PgConnection
) -> ServiceResult<StandardFav> {
    use crate::schema::standard_fav::dsl::*;

    let new_standard_favorite: InsertableStandardFav = data.into();

    let flag_found_favorite = standard_fav
        .filter(standard_uuid.eq(&new_standard_favorite.standard_uuid)
        .and(user_uuid.eq(&new_standard_favorite.user_uuid)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    match flag_found_favorite as i32 {
        0 => {
            let inserted_standard_favorite: StandardFav = diesel::insert_into(standard_fav)
                .values(&new_standard_favorite)
                .get_result(conn)?;
            Ok(inserted_standard_favorite)
        },
        _ => Err(ServiceError::BadRequest("This favorite name is already with the standard.".to_string())),
    }
}
