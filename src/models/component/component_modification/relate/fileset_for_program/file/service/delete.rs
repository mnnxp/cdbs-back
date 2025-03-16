use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::service::update::change_updated_at;
use crate::models::component::{
    component_modification::fileset_for_program::file::model::DelModificationFileFromFilesetData,
    component_modification::relate::fileset_for_program::util::{
        get_component_by_fileset, get_modification_by_fileset
    },
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет файлы из набора файлов модификации компонента.
pub(crate) fn del_file_from_fileset(
    logged_user_uuid: &Uuid,
    data: &DelModificationFileFromFilesetData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)
    let target_component_uuid = get_component_by_fileset(&data.fileset_uuid, conn)?;
    let target_modification_uuid = get_modification_by_fileset(&data.fileset_uuid, conn)?;
    check_access_component_for_user(
        logged_user_uuid,
        &target_component_uuid,
        &need_access_level,
        conn
    )?;

    match &data.file_uuids.is_empty() {
        true => Ok(false),
        false => {
            // set flags for delete files in storage
            delete_file_by_uuids(&data.file_uuids, conn)?;
            // update the updated_at for component and modification if files of set of files has been deleted
            change_updated_at(&target_component_uuid, Some(&target_modification_uuid), conn)?;
            delete_file_link_row(data, conn)
        },
    }
}

/// Remove the relate of the file to the modification
fn delete_file_link_row(
    data: &DelModificationFileFromFilesetData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let count = diesel::delete(modification_file_from_fileset::modification_file_from_fileset)
        .filter(modification_file_from_fileset::fileset_uuid.eq(&data.fileset_uuid)
        .and(modification_file_from_fileset::file_uuid.eq_any(&data.file_uuids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Error delete files of fileset: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count > 0)
}
