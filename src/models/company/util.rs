use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use diesel::prelude::*;
use uuid::Uuid;

/// Checking whether the company has a supplier's status
pub(crate) fn check_is_supplier(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    use crate::schema::company_ref::dsl::*;

    let get_company_status = company_ref
        .filter(uuid.eq(target_company_uuid))
        .select(is_supplier)
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
