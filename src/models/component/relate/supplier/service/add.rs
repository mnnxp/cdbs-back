use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::supplier::model::{
    SupplierComponent, IptSupplierComponentData, InsertableSupplierComponent
};
use crate::models::component::util::check_is_base_with_err;
use crate::models::company::util::check_is_supplier;
use diesel::prelude::*;
use uuid::Uuid;

/// Add related supplier for component base
pub(crate) fn add_component_base_supplier(
    logged_user_uuid: &Uuid,
    data: &IptSupplierComponentData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    // if logged user can view base component,
    // they can add company to supplier list
    let need_access_level = 3; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    // checking if a component is basic
    check_is_base_with_err(&data.component_uuid, conn)?;

    // checking if the company is a supplier
    check_is_supplier(&data.company_uuid, conn)?;

    // add row in database
    add_component_supplier_company(
        data,
        conn
    )
}

/// Insert row in supplier_to_component table
/// Warning: without check access
pub(crate) fn add_component_supplier_company(
    data: &IptSupplierComponentData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::supplier_to_component::dsl::*;

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
