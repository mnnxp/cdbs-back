use crate::errors::ServiceResult;
use crate::models::company::company_fav::model::CompanyFav;
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyFav {
    /// Count subscribers for company
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(company_fav::company_fav
            .filter(company_fav::user_uuid.eq(target_user_uuid)
            .and(company_fav::is_enabled.eq(true)))
            .execute(conn)? as i32)
    }
}
