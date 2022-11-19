use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::supplier::model::DelSuppliersComponentData;
use diesel::prelude::*;
use uuid::Uuid;

/// Remove related suppliers from component
/// delete rows in supplier_to_component table
pub(crate) fn del_suppliers_component(
    logged_user_uuid: &Uuid,
    data: &DelSuppliersComponentData,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::schema::supplier_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

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

/// Remove all suppliers component
/// delete all rows in supplier_to_component table
pub(crate) fn clear_suppliers_component(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::models::component::access::util::check_is_owner_with_err;
    use crate::schema::supplier_to_component::dsl::*;

    // return error if logged user not ownership component
    check_is_owner_with_err(
        logged_user_uuid,
        target_component_uuid,
        conn
    )?;

    let del_count = diesel::delete(supplier_to_component
        .filter(component_uuid.eq(target_component_uuid)))
        .execute(conn);

    match del_count {
        Ok(count) => {
            debug!("Delete {:?} suppliers component", count);
            Ok(count as i32)
        },
        Err(err) => {
            debug!("Failed delete related suppliers to component: {:?}", err);
            Err(ServiceError::BadRequest("Failed delete related suppliers to component".to_string()))
        },
    }
}
