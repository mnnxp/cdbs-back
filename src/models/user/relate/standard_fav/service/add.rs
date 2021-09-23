use crate::errors::ServiceResult;
use crate::models::user::standard_fav::model::{
    StandardFav,
    IptStandardFavData,
    InsertableStandardFav,
};
use crate::schema::standard_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_standard_fav(
    data: IptStandardFavData,
    conn: &PgConnection,
) -> ServiceResult<StandardFav> {
    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(standard_fav)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .get_result(conn);

    let user_standard_fav: StandardFav = match check_fav {
        Ok(fav) => fav,
        Err(_) => {
            // add flag and date created
            let insertable_fav: InsertableStandardFav = data.into();

            diesel::insert_into(standard_fav)
                .values(insertable_fav)
                .get_result(conn)?
        },
    };

    debug!("User favorite standard: {:#?}", user_standard_fav);

    Ok(user_standard_fav)
}
