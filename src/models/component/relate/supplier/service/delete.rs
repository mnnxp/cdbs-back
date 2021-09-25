use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::supplier::model::DelSupplierToComponentData;
use diesel::prelude::*;
// use uuid::Uuid;

/// Remove related suppliers from component
/// delete rows in supplier_to_component table
pub(crate) fn del_suppliers_component(
    data: &DelSupplierToComponentData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::supplier_to_component::dsl::*;

    let del_count = diesel::delete(supplier_to_component
        .filter(component_uuid.eq(&data.component_uuid)
        .and(company_uuid.eq_any(&data.companies_uuids))))
        .execute(conn);

    match del_count {
        Ok(count) => Ok(count as i32),
        Err(err) => {
            debug!("Failed delete related suppliers to component: {:?}", err);
            Err(ServiceError::BadRequest("Failed delete related suppliers to component".to_string()))
        },
    }
}
