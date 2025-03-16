use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::supplier::model::DelSuppliersComponentData;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет поставщиков компонента по UUIDs.
pub(crate) fn del_suppliers_component(
    logged_user_uuid: &Uuid,
    data: &DelSuppliersComponentData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
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
        Ok(count) => Ok(count),
        Err(err) => {
            debug!("Failed delete related suppliers to component: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedDeleteSuppliersComponent))
        },
    }
}

/// Remove all suppliers component
/// delete all rows in supplier_to_component table
pub(crate) fn clear_suppliers_component(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
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
            Ok(count)
        },
        Err(err) => {
            debug!("Failed delete related suppliers to component: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedDeleteSuppliersComponent))
        },
    }
}
