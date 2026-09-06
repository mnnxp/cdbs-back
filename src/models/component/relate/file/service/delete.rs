use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::relate::file::model::DelComponentFileData;
use crate::models::component::service::update::change_updated_at;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::schema::file_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет файл компонента.
pub(crate) fn delete_component_file(
    logged_user_uuid: &Uuid,
    data: &DelComponentFileData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &data.component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let del_file = diesel::delete(file_to_component)
        .filter(
            component_uuid
                .eq(&data.component_uuid)
                .and(file_uuid.eq(&data.file_uuid)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail delete row: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match del_file {
        // not found file
        0 => Ok(false),
        // set flag for delete file in storage
        _ => {
            change_updated_at(&data.component_uuid, None, conn)?;
            delete_file_by_uuid(&data.file_uuid, conn)
        }
    }
}
