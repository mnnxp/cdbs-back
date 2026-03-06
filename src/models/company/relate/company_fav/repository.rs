use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_fav::model::CompanyFav;
use crate::schema::company_fav::dsl as company_fav;
use crate::subscribers_count;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyFav {
    /// Count subscribers for company
    pub(crate) fn get_count_followers_by_uuid(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        subscribers_count!(company_fav, company_uuid, target_company_uuid, conn)
    }
}
