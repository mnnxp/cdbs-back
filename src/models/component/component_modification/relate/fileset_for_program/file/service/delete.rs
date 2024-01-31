use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    component_modification::fileset_for_program::file::model::DelModificationFileFromFilesetData,
    component_modification::relate::fileset_for_program::util::get_component_by_fileset,
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

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&data.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    match &data.file_uuids.is_empty() {
        true => Ok(false),
        false => {
            // set flags for delete files in storage
            delete_file_by_uuids(&data.file_uuids, conn)?;

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
