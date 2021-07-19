use crate::errors::ServiceResult;
use crate::models::company::model::{InsertableCompany, SlimCompany, Company, CompanyData};
use diesel::prelude::*;

pub(crate) fn create_company(company_data: CompanyData, conn: &PgConnection) -> ServiceResult<SlimCompany> {
    use crate::schema::company_ref::dsl::company_ref;

    let company: InsertableCompany = company_data.into();
    let inserted_company: Company = diesel::insert_into(company_ref).values(&company).get_result(conn)?;
    Ok(inserted_company.into())
}
