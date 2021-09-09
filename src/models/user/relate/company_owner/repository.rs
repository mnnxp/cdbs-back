use crate::errors::ServiceResult;
use crate::models::company::model::ShowCompanyShort;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowCompanyShort {
    /// get list subscribers for company
    pub fn get_by_user_uuid(
        target_user_uuid: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        let target_companies_uuids = company_ref::company_ref
            .filter(company_ref::uuid_user.eq(target_user_uuid))
            .select(company_ref::uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowCompanyShort::get_list_by_uuids(
            &target_companies_uuids,
            target_user_uuid,
            set_id_lang,
            conn,
        )
    }

    /// Count subscribers for company
    pub fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(company_ref::company_ref
            .filter(company_ref::uuid_user.eq(target_user_uuid))
            .execute(conn)? as i32)
    }
}
