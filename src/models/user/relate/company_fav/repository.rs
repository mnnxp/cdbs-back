use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_fav::model::CompanyFav;
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyFav {
    /// Count subscribers for company
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        let count = company_fav::company_fav
            .filter(
                company_fav::user_uuid
                    .eq(target_user_uuid)
                    .and(company_fav::is_enabled.eq(true)),
            )
            .execute(conn)
            .map_err(|err| {
                debug!("Fail count company_fav: {:?} ", err);
                ServiceError::InternalServerError
            })?;
        Ok(count as i32)
    }
}
