use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult,
};
use async_graphql::Context;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use uuid::Uuid;

/// Search company represents for company by company uuid
pub(crate) fn get_by_company_uuid(
    context: &Context<'_>,
    target_company_uuid: &Uuid,
) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let result: Vec<CompanyRepresentAndRelatedData> = CompanyRepresentAndRelatedData::get_list_represents_by_company_uuid(
        target_company_uuid,
        &set_id_lang,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

/// Search company represents by represent uuid
pub(crate) fn get_represent_by_uuids(
    context: &Context<'_>,
    target_represents_uuids: &[Uuid],
) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for represent
    let result: Vec<CompanyRepresentAndRelatedData> = CompanyRepresentAndRelatedData::get_list_represents_by_uuids(
        target_represents_uuids,
        &set_id_lang,
        conn
    ).expect("Error loading company and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
