use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::access::util::check_company_access;
use crate::models::company::supplier_component::model::DelCompanyOfSuppliersData;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет компанию из списка поставщиков.
pub(crate) fn del_company_of_suppliers(
    logged_user_uuid: &Uuid,
    data: &DelCompanyOfSuppliersData,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    use crate::schema::supplier_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    let del_count = diesel::delete(supplier_to_component
        .filter(company_uuid.eq(&data.company_uuid)
        .and(component_uuid.eq(&data.component_uuid))))
        .execute(conn);

    match del_count {
        Ok(0) => {
            debug!("Not found company in suppliers component: {:?}", del_count);
            Ok(false)
        },
        Ok(x) => {
            debug!("Delete company of supplier list: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed delete company of suppliers component: {:?}", err);
            Err(ServiceError::BadRequest("Failed delete company of suppliers component".to_string()))
        },
    }
}
