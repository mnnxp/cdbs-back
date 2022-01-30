use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::service::delete::delete_modifications_files_by_component;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::{
    component_ref::dsl as component_ref,
    file_to_component::dsl as file_to_component,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component(
    logged_user_uuid: &Uuid,
    del_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    // set flags for component files
    delete_component_files(del_component_uuid, conn)?;
    // set flags for component modifications and filesets files
    delete_modifications_files_by_component(del_component_uuid, conn)?;

    diesel::delete(component_ref::component_ref
        .filter(component_ref::user_uuid.eq(logged_user_uuid)
        .and(component_ref::uuid.eq(del_component_uuid))))
        .returning(component_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete component: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Set the delete flags for all files associated with the component
pub(crate) fn delete_component_files(
    component_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let del_file_uuids = file_to_component::file_to_component
        .filter(file_to_component::component_uuid.eq(component_uuid))
        .select(file_to_component::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
