use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::license::model::IptComponentLicenseData;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет лицензию на компонент.
pub(crate) fn del_component_license(
    logged_user_uuid: &Uuid,
    data: &IptComponentLicenseData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    use crate::schema::license_to_component::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &data.component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    if data.license_id < 0 {
        // return error if not correct license id
        return Err(get_err_msg(ErrorMessage::ErrorIncorrectId));
    }

    diesel::delete(license_to_component)
        .filter(
            component_uuid
                .eq(&data.component_uuid)
                .and(license_id.eq(&data.license_id)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted license: {:?}", err);
            ServiceError::InternalServerError
        })
}
