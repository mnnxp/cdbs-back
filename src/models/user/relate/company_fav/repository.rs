use crate::errors::ServiceResult;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::company::model::ShowCompanyShort;
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowCompanyShort {
    /// get list subscribers for company
    pub(crate) fn _get_by_user_uuid(
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        let target_companies_uuids = company_fav::company_fav
            .filter(company_fav::user_uuid.eq(target_user_uuid)
            .and(company_fav::is_enabled.eq(true)))
            .select(company_fav::company_uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowCompanyShort::get_list_by_uuids(
            &target_companies_uuids,
            target_user_uuid,
            set_lang_id,
            conn,
        )
    }
}

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
