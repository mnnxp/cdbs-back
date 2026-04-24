use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::service::update::change_updated_at;
use crate::models::component::component_modification::{
    fileset_for_program::model::{InsertableFilesetProgram, IptFilesetProgramData},
    util::get_component_by_modification,
};
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

/// Создает набор файлов для модификации компонента.
/// Файлы, необходимые для работы конкретного ПО (САПР), загружаются в наборы файлов для этого ПО.
/// Также наборы файлов используются для настройки интеграции с различными системами CAD/CAM и т. д.
pub(crate) fn create_modification_fileset(
    logged_user_uuid: &Uuid,
    arg: &IptFilesetProgramData,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    let target_component_uuid = get_component_by_modification(&arg.modification_uuid, conn)?;
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &target_component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let find_fileset = &fileset_for_program::fileset_for_program
        .filter(
            fileset_for_program::modification_uuid
                .eq(&arg.modification_uuid)
                .and(fileset_for_program::program_id.eq(&arg.program_id)),
        )
        .select(fileset_for_program::uuid)
        .limit(1)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Not found target fileset_for_program: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match find_fileset.first() {
        Some(x) => {
            debug!(
                "The modification has a set of files for this program: {:?}",
                x
            );
            Ok(*x)
        }
        None => {
            // update the updated_at for component and modification if new fileset are added
            change_updated_at(&target_component_uuid, Some(&arg.modification_uuid), conn)?;
            let data: InsertableFilesetProgram = arg.into();
            diesel::insert_into(fileset_for_program::fileset_for_program)
                .values(&data)
                .returning(fileset_for_program::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Failed insert fileset_for_program: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}
