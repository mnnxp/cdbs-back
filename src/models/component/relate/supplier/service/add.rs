use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::supplier::model::{
    SupplierComponent,
    IptSupplierComponentData,
    InsertableSupplierComponent
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_component_supplier(
    data: IptSupplierComponentData,
    conn: &PgConnection
) -> ServiceResult<SupplierComponent> {
    use crate::schema::supplier_to_component::dsl::*;

    let new_component_supplier: InsertableSupplierComponent = data.into();

    let flag_found_supplier = supplier_to_component
        .filter(component_uuid.eq(&new_component_supplier.component_uuid)
        .and(company_uuid.eq(&new_component_supplier.company_uuid)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_supplier START SEARCH ={:?}", flag_found_supplier);

    match flag_found_supplier as i32 {
        0 => {
            Ok(diesel::insert_into(supplier_to_component)
                .values(&new_component_supplier)
                .get_result::<SupplierComponent>(conn)?)
        },
        _ => Err(ServiceError::BadRequest("This supplier name is already with the component.".to_string())),
    }
}
