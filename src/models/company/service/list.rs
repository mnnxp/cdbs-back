use crate::errors::ServiceResult;
use crate::models::company::model::{ShowCompanyShort, CompanyAndRelatedData};
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn find_companies(
    target_uuids_companies: &[Uuid],
    target_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowCompanyShort>> {


    let result: Vec<ShowCompanyShort> = ShowCompanyShort::get_list_by_uuids(
        target_uuids_companies,
        target_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Companies data: {:#?}", result);

    Ok(result)
}

/// Gets company with related data, with translate by uuid
pub(crate) fn find_by_uuid(
    target_company_uuid: &Uuid,
    target_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {


    // collect data for company
    let result: CompanyAndRelatedData = CompanyAndRelatedData::collect_related_data(
        target_company_uuid,
        target_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading company and collect related data");

    debug!("Company data: {:#?}", result);

    Ok(result)
}
