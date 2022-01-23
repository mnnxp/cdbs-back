use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    component_modification::{
        fileset_for_program::model::DelFilesetProgramData,
        relate::fileset_for_program::util::get_component_by_fileset,
    },
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete a set of files for the program
pub(crate) fn del_modification_fileset(
    logged_user_uuid: &Uuid,
    data: &DelFilesetProgramData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&data.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let files_of_set = modification_file_from_fileset::modification_file_from_fileset
        .filter(modification_file_from_fileset::fileset_uuid.eq(&data.fileset_uuid))
        .select(modification_file_from_fileset::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Error get fileset: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // debug!("Delete fileset: {:?}", del_fileset);
    if !files_of_set.is_empty() {
        // set flags for delete files in storage
        delete_file_by_uuids(&files_of_set, conn)?;
    }

    delete_fileset_row(data, conn)
}

fn delete_fileset_row(
    data: &DelFilesetProgramData,
    conn: &PgConnection,
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
