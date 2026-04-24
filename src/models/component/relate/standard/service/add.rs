use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::standard::model::{
    InsertableStandardToComponent, IptStandardToComponentData,
};
use crate::schema::standard_to_component::dsl as standard_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Прикрепляет стандарт к компоненту.
pub(crate) fn add_standard_to_component(
    logged_user_uuid: &Uuid,
    data: &IptStandardToComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &data.component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let found_standard = standard_to_component::standard_to_component
        .filter(
            standard_to_component::component_uuid
                .eq(data.component_uuid)
                .and(standard_to_component::standard_uuid.eq(data.standard_uuid)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check standards component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if found_standard > 0 {
        return Err(get_err_msg(
            ErrorMessage::StanardIsAlreadyAssociatedWithComponent,
        ));
    }

    let new_component_standard: InsertableStandardToComponent = data.into();

    diesel::insert_into(standard_to_component::standard_to_component)
        .values(&new_component_standard)
        .returning(standard_to_component::component_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed add standard component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(true)
}
