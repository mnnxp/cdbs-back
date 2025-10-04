use crate::errors::{ServiceError, ServiceResult};
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет стандарт из списка избранного авторизованного пользователя.
pub(crate) fn delete_standard_fav(
    logged_user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(standard_fav::standard_fav)
        .filter(
            standard_fav::standard_uuid
                .eq(standard_uuid)
                .and(standard_fav::user_uuid.eq(logged_user_uuid))
                .and(standard_fav::is_enabled.eq(true)),
        ) // <-- active favorite
        .set(standard_fav::is_enabled.eq(false)) // <-- off favorite standard
        .returning(standard_fav::is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
