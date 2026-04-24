use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::search::order::Paginate;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список поставщиков компонента
pub(crate) fn get_component_suppliers(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        component_uuid,
        AccessOperation::Read,
        conn,
    )?;
    ComponentSupplierRelatedData::by_component_uuid(component_uuid, paginate, conn)
}
