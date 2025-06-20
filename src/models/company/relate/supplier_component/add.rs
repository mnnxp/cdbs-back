use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::access::util::check_company_access;
use crate::models::component::access::company::manage::give_company_top_access_component;
use crate::models::component::supplier::model::IptSupplierComponentData;
use crate::models::component::supplier::service::add::{
    add_component_base_supplier, add_component_supplier_company,
};
use crate::models::component::supplier::service::delete::clear_suppliers_component;
use crate::models::component::util::check_is_base;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет компанию в список поставщиков компонента.
pub(crate) fn add_company_to_suppliers(
    logged_user_uuid: &Uuid,
    data: &IptSupplierComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    add_component_base_supplier(logged_user_uuid, data, conn)
}

/// Устанавливает компанию в качестве основного поставщика компонента.
pub(crate) fn set_company_owner_supplier(
    logged_user_uuid: &Uuid,
    data: &IptSupplierComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    // check on no base component
    if check_is_base(&data.component_uuid, conn)? {
        return Err(get_err_msg(ErrorMessage::DoesNotWorkForBaseComponent));
    };

    // clear out supplier component list
    // and second check component ownership :)
    clear_suppliers_component(logged_user_uuid, &data.component_uuid, conn)?;

    // add top access for company
    // return err if logged user not owner component
    give_company_top_access_component(
        logged_user_uuid,
        &data.component_uuid,
        &data.company_uuid,
        conn,
    )?;

    // add new supplier component without check access
    add_component_supplier_company(data, conn)
}
