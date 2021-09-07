use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::company::model::{ShowCompanyShort, CompanyAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_companies(
    context: &Context<'_>,
    target_uuids_companies: &[Uuid],
    target_uuid_user: &Uuid,
) -> ServiceResult<Vec<ShowCompanyShort>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let result: Vec<ShowCompanyShort> = ShowCompanyShort::get_list_by_uuids(
        target_uuids_companies,
        target_uuid_user,
        &set_id_lang,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Companys data: {:#?}", result);

    Ok(result)
}

/// Gets company with related data, with translate by uuid
pub(crate) fn find_by_uuid(
    context: &Context<'_>,
    target_company_uuid: &Uuid,
    target_user_uuid: &Uuid,
) -> ServiceResult<CompanyAndRelatedData> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for company
    let result: CompanyAndRelatedData = CompanyAndRelatedData::collect_related_data(
        target_company_uuid,
        target_user_uuid,
        &set_id_lang,
        conn
    ).expect("Error loading company and collect related data");

    debug!("Company data: {:#?}", result);

    Ok(result)
}
