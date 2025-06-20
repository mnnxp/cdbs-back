use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_fav::model::CompanyFav;
// use crate::models::user::model::ShowUserShort;
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyFav {
    /// Count subscribers for company
    pub(crate) fn get_count_followers_by_uuid(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        let count = company_fav::company_fav
            .filter(company_fav::company_uuid.eq(target_company_uuid))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed count followers: {:?}", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
