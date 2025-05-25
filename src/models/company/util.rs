use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking whether the company has a supplier's status
pub(crate) fn check_is_supplier(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    let get_company_status = company_ref::company_ref
        .filter(company_ref::uuid.eq(target_company_uuid))
        .select(company_ref::is_supplier)
        .first::<bool>(conn);

    match get_company_status {
        Ok(true) => Ok(true),
        Ok(false) => Err(get_err_msg(ErrorMessage::CompanyIsNotSupplier)),
        Err(err) => {
            debug!("Failed check data: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedCheckData))
        },
    }
}

/// Returns the owner-user Uuid for the company
pub(crate) fn get_company_owner(
    company_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<Uuid> {
    company_ref::company_ref
        .filter(company_ref::uuid.eq(company_uuid))
        .select(company_ref::user_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Not found owner-user for company: {:?}", err);
            ServiceError::InternalServerError
        })
}
