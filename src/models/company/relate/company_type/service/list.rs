use crate::errors::ServiceResult;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use diesel::PgConnection;

/// Gets types data for company
pub(crate) fn get_types_for_company(
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
    CompanyTypeTranslateList::get_company_types(set_lang_id, conn)
}
