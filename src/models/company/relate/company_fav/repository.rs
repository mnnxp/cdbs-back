use crate::errors::ServiceResult;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyFav {
    /// get list subscribers for company
    pub fn get_list_followers_by_uuid(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let target_list_user_uuid = company_fav::company_fav
            .filter(company_fav::company_uuid.eq(target_company_uuid))
            .select(company_fav::user_uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowUserShort::get_list_by_uuids(&target_list_user_uuid, conn)
    }

    /// Count subscribers for company
    pub fn get_count_followers_by_uuid(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(company_fav::company_fav
            .filter(company_fav::company_uuid.eq(target_company_uuid))
            .execute(conn)? as i32)
    }
}
