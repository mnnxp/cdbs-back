use crate::config;
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::FileByExtArg;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Retund default component
pub(crate) fn get_root_component_uuid() -> Uuid {
    config::root_component_uuid()
}

/// Checking whether the component has flag is_base
/// return true or false
pub(crate) fn check_is_base(component_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    component_ref::component_ref
        .filter(component_ref::uuid.eq(component_uuid))
        .select(component_ref::is_base)
        .get_result::<bool>(conn)
        .map_err(|err| {
            debug!("Failed get component status {:?}", err);
            get_err_msg(ErrorMessage::FailedCheckData)
        })
}

/// Checking whether the component has flag is_base
/// return err if not base
pub(crate) fn check_is_base_with_err(
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let get_component_status = component_ref::component_ref
        .filter(component_ref::uuid.eq(component_uuid))
        .select(component_ref::is_base)
        .first::<bool>(conn);

    match get_component_status {
        Ok(true) => Ok(true),
        Ok(false) => Err(get_err_msg(ErrorMessage::ComponentIsNotStandard)),
        _ => Err(get_err_msg(ErrorMessage::FailedCheckData)),
    }
}

// Get files uuids for a component by uuid with filter by extension
pub(crate) fn get_files_by_ext(
    component_uuid: &Uuid,
    arg: &FileByExtArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::file_ref::dsl as file_ref;
    use crate::schema::file_to_component::dsl as file_to_component;

    let file_uuids = file_to_component::file_to_component
        .select(file_to_component::file_uuid)
        .filter(file_to_component::component_uuid.eq(component_uuid))
        .limit(1000)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get component files {:?}", err);
            ServiceError::InternalServerError
        })?;

    file_ref::file_ref
        .select(file_ref::uuid)
        .filter(
            file_ref::uuid.eq_any(file_uuids).and(
                file_ref::id_ext.eq(&arg.ext_id).and(
                    file_ref::is_hidden
                        .eq(false)
                        .and(file_ref::is_delete.eq(false)),
                ),
            ),
        )
        .limit(arg.limit)
        .offset(arg.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get image files {:?}", err);
            ServiceError::InternalServerError
        })
}
