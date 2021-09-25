use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::supplier::model::{
    SupplierComponent, IptSupplierComponentData, InsertableSupplierComponent
};
use crate::models::component::util::check_is_standard;
use crate::models::company::util::check_is_supplier;
use diesel::prelude::*;
// use uuid::Uuid;

/// Add related suppliers from component
/// insert row in supplier_to_component table
pub(crate) fn add_component_supplier(
    data: &IptSupplierComponentData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::supplier_to_component::dsl::*;

    // checking if a component is basic
    check_is_standard(&data.component_uuid, conn)?;

    // checking if the company is a supplier
    check_is_supplier(&data.company_uuid, conn)?;

    let found_supplier = supplier_to_component
        .filter(component_uuid.eq(&data.component_uuid)
        .and(company_uuid.eq(&data.company_uuid)))
        .execute(conn);

    match found_supplier {
        Ok(found) => {
            if found > 0 {
                return Err(ServiceError::BadRequest(
                    "This supplier is already with the component".to_string()
                ))
            }

            let new_component_supplier: InsertableSupplierComponent = data.into();

            match diesel::insert_into(supplier_to_component)
                .values(&new_component_supplier)
                .get_result::<SupplierComponent>(conn) {
                Ok(_) => Ok(true),
                Err(err) => {
                    debug!("Failed add supplier component: {:?}", err);
                    Err(ServiceError::BadRequest("Failed add supplier component".to_string()))
                },
            }
        },
        Err(err) => {
            debug!("Failed check suppliers component: {:?}", err);
            Err(ServiceError::BadRequest("Failed check suppliers component".to_string()))
        },
    }
}
