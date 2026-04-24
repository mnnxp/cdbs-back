use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_represent::model::{
    CompanyRepresent, InsertableCompanyRepresent, IptCompanyRepresentData,
};
use crate::schema::company_represent_ref::dsl::company_represent_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавление информации о представительстве компании.
pub(crate) fn create_company_represent(
    logged_user_uuid: &Uuid,
    data: &IptCompanyRepresentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &data.company_uuid,
        AccessOperation::Write,
        conn,
    )?;

    // check_is_supplier(&data.company_uuid, conn)?;

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
