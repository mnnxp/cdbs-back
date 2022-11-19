use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::company_represent::model::{
    CompanyRepresent,
    IptCompanyRepresentData,
    InsertableCompanyRepresent,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_company_represent(
    logged_user_uuid: &Uuid,
    data: &IptCompanyRepresentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_represent_ref::dsl::company_represent_ref;

    check_is_owner_with_err(
        logged_user_uuid,
        &data.company_uuid,
        conn
    )?;

    crate::models::company::util::check_is_supplier(&data.company_uuid, conn)?;

    let company_represent: InsertableCompanyRepresent = data.into();
    diesel::insert_into(company_represent_ref)
        .values(&company_represent)
        .get_result::<CompanyRepresent>(conn)
        .map_err(|err| {
            debug!("Failed insert represent types: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(true)
}
