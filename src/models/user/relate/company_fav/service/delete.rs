use crate::errors::{ServiceError, ServiceResult};
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет компанию из списка избранного авторизованного пользователя.
pub(crate) fn delete_company_fav(
    logged_user_uuid: &Uuid,
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if have need row, just update is_enabled to false
    let del_fav = diesel::update(company_fav::company_fav)
        .filter(
            company_fav::company_uuid
                .eq(company_uuid)
                .and(company_fav::user_uuid.eq(logged_user_uuid))
                .and(company_fav::is_enabled.eq(true)),
        ) // <-- active favorite
        .set(company_fav::is_enabled.eq(false)) // <-- off favorite company
        .returning(company_fav::is_enabled)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed delete fav company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(!del_fav)
}
