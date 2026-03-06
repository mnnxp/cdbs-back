use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::util::check_is_supplier;
use crate::models::component::supplier::model::{
    InsertableSupplierComponent, IptSupplierComponentData, SupplierComponent,
};
use crate::models::component::util::check_is_base_with_err;
use crate::schema::supplier_to_component::dsl as supplier_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Add related supplier for component base
pub(crate) fn add_component_base_supplier(
    logged_user_uuid: &Uuid,
    data: &IptSupplierComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // if logged user can view base component,
    // they can add company to supplier list
    let need_access_level = 3; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        need_access_level,
        conn,
    )?;

    // checking if a component is basic
    check_is_base_with_err(&data.component_uuid, conn)?;

    // checking if the company is a supplier
    check_is_supplier(&data.company_uuid, conn)?;

    // add row in database
    add_component_supplier_company(data, conn)
}

/// Insert row in supplier_to_component table
/// Warning: without check access
pub(crate) fn add_component_supplier_company(
    data: &IptSupplierComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let count_suppliers = supplier_to_component::supplier_to_component
        .filter(
            supplier_to_component::component_uuid
                .eq(&data.component_uuid)
                .and(supplier_to_component::company_uuid.eq(&data.company_uuid)),
        )
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check suppliers component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match count_suppliers {
        0 => {
            let insert_data: InsertableSupplierComponent = data.into();
            diesel::insert_into(supplier_to_component::supplier_to_component)
                .values(&insert_data)
                .get_result::<SupplierComponent>(conn)
                .map_err(|err| {
                    debug!("Failed add supplier component: {:?}", err);
                    ServiceError::InternalServerError
                })?;
            Ok(true)
        }
        _ => {
            diesel::update(supplier_to_component::supplier_to_component)
                .filter(
                    supplier_to_component::component_uuid
                        .eq(&data.component_uuid)
                        .and(supplier_to_component::company_uuid.eq(&data.company_uuid)),
                )
                .set(supplier_to_component::description.eq(&data.description))
                .get_result::<SupplierComponent>(conn)
                .map_err(|err| {
                    debug!("Failed add supplier component: {:?}", err);
                    ServiceError::InternalServerError
                })?;
            Ok(true)
        }
    }
}
