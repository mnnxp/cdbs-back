use crate::errors::ServiceResult;
use crate::models::company::company_represent::model::{
    CompanyRepresent, CompanyRepresentData, InsertableCompanyRepresent, SlimCompanyRepresent,
};
use diesel::prelude::*;

pub fn create_company_represent(
    company_represent_data: CompanyRepresentData,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyRepresent> {
    use crate::schema::company_represent_ref::dsl::company_represent_ref;

    let company_represent: InsertableCompanyRepresent = company_represent_data.into();
    let inserted_company_represent: CompanyRepresent = diesel::insert_into(company_represent_ref)
        .values(&company_represent)
        .get_result(conn)?;
    Ok(inserted_company_represent.into())
}
