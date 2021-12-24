use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    component_modification::{
        modification_file_from_fileset::model::DelModificationFileFromFilesetData,
        relate::fileset_for_program::util::get_component_by_fileset,
    },
    access::util::check_access_component_for_user,
};
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete a set of files for the program
pub(crate) fn del_file_from_fileset(
    logged_user_uuid: &Uuid,
    data: &DelModificationFileFromFilesetData,
    conn: &PgConnection,
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
            // todo!(here delete files of file_ref table and of storage)
            delete_file_row(data, conn)
        },
    }
}

fn delete_file_row(
    data: &DelModificationFileFromFilesetData,
    conn: &PgConnection,
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
