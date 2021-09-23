use crate::errors::ServiceResult;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use diesel::PgConnection;
use uuid::Uuid;

/// Search company represents for company by company uuid
pub(crate) fn get_by_company_uuid(
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {


    let result: Vec<CompanyRepresentAndRelatedData> = CompanyRepresentAndRelatedData::get_list_represents_by_company_uuid(
        target_company_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

/// Search company represents by represent uuid
pub(crate) fn get_represent_by_uuids(
    target_represents_uuids: &[Uuid],
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {


    // collect data for represent
    let result: Vec<CompanyRepresentAndRelatedData> = CompanyRepresentAndRelatedData::get_list_represents_by_uuids(
        target_represents_uuids,
        set_lang_id,
        conn
    ).expect("Error loading company and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
