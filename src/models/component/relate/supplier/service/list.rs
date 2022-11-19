use crate::errors::ServiceResult;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
use uuid::Uuid;

/// Get suppliers for component
pub(crate) fn get_component_suppliers(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        component_uuid,
        &need_access_level,
        conn
    )?;

    ComponentSupplierRelatedData::by_component_uuid(component_uuid, conn)
}
