use crate::errors::ServiceResult;
use crate::models::company::company_represent::model::{
    CompanyRepresent,
    CompanyRepresentData,
    IptCompanyRepresentData,
    InsertableCompanyRepresent,
    SlimCompanyRepresent,
};
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_company_represent(
    logged_user_uuid: &Uuid,
    data: &IptCompanyRepresentData,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyRepresent> {
    use crate::schema::company_represent_ref::dsl::company_represent_ref;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::company::access::util::check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    crate::models::company::util::check_is_supplier(&data.company_uuid, conn)?;

    let company_represent_data: CompanyRepresentData = data.into();

    let company_represent: InsertableCompanyRepresent = company_represent_data.into();
    let inserted_company_represent: CompanyRepresent = diesel::insert_into(company_represent_ref)
        .values(&company_represent)
        .get_result(conn)?;
    Ok(inserted_company_represent.into())
}
