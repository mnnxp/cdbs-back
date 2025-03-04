use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_modification::fileset_for_program::file::repository::get_file_uuids_by_fileset_uuid;
use crate::models::component::service::update::change_updated_at;
use crate::models::component::{
    component_modification::fileset_for_program::model::DelFilesetProgramData,
    component_modification::relate::fileset_for_program::util::get_component_by_fileset,
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет набор файлов из модификации компонента.
pub(crate) fn del_modification_fileset(
    logged_user_uuid: &Uuid,
    data: &DelFilesetProgramData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)
    let target_component_uuid = get_component_by_fileset(&data.fileset_uuid, conn)?;
    check_access_component_for_user(
        logged_user_uuid,
        &target_component_uuid,
        &need_access_level,
        conn
    )?;

    let files_of_set = get_file_uuids_by_fileset_uuid(&data.fileset_uuid, &[], conn)?;

    // debug!("Delete fileset: {:?}", del_fileset);
    if !files_of_set.is_empty() {
        // set flags for delete files in storage
        delete_file_by_uuids(&files_of_set, conn)?;
        // update the updated_at for component and modification if a new set of files has been deleted
        change_updated_at(&target_component_uuid, Some(&data.modification_uuid), conn)?;
    }

    delete_fileset_row(data, conn)
}

fn delete_fileset_row(
    data: &DelFilesetProgramData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let count = diesel::delete(fileset_for_program::fileset_for_program)
        .filter(fileset_for_program::uuid.eq(&data.fileset_uuid)
        .and(fileset_for_program::modification_uuid.eq(&data.modification_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Error delete fileset: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count > 0)
}

/// Set the delete flags for all files of filesets associated with the component modifications
pub(crate) fn delete_filesets_files_by_modifications(
    modification_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // get filesets related with component modifications
    let fileset_uuids = fileset_for_program::fileset_for_program
        .select(fileset_for_program::uuid)
        .filter(fileset_for_program::modification_uuid.eq_any(modification_uuids))
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets modifications for component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    let del_file_uuids = modification_file_from_fileset::modification_file_from_fileset
        .select(modification_file_from_fileset::file_uuid)
        .filter(modification_file_from_fileset::fileset_uuid.eq_any(&fileset_uuids))
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
