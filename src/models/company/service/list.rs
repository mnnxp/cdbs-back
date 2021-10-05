use crate::errors::ServiceResult;
use crate::models::company::model::{ShowCompanyShort, CompanyAndRelatedData};
use crate::models::company::access::util::check_company_access;
use diesel::PgConnection;
use uuid::Uuid;

/// Gets companies with allow access for user
pub(crate) fn find_companies(
    logged_user_uuid: &Uuid,
    target_companies_uuids: &[Uuid],
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowCompanyShort>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for all companies
    for cy_uuid in target_companies_uuids {
        check_company_access(
            logged_user_uuid,
            cy_uuid,
            &need_access_level,
            conn
        )?;
    }

    let result: Vec<ShowCompanyShort> = ShowCompanyShort::get_list_by_uuids(
        target_companies_uuids,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Companies data: {:#?}", result);

    Ok(result)
}

/// Gets company with related data, with translate by uuid
pub(crate) fn find_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        logged_user_uuid,
        target_company_uuid,
        &need_access_level,
        conn
    )?;

    // collect data for company
    let result: CompanyAndRelatedData = CompanyAndRelatedData::collect_related_data(
        target_company_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading company and collect related data");

    debug!("Company data: {:#?}", result);

    Ok(result)
}
