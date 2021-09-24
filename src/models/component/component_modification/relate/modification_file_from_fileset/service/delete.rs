use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::modification_file_from_fileset::model::{
    ModificationFileFromFileset,
    DelModificationFileFromFilesetData,
};
// use crate::models::relate_ref::file::service::delete::delete_rows_by_uuids;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete a set of files for the program
/// not delete row in file_ref and file in storage
pub(crate) fn del_file_from_fileset(
    _logged_user_uuid: &Uuid, // <-- todo!(check access)
    data: &DelModificationFileFromFilesetData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    match &data.file_uuids {
        files_of_set if files_of_set.is_empty() => Ok(false), // <-- not found files for delete
        _ => {
            // delete row about file of fileset from modification_file_from_fileset table
            if delete_file_row_of_fileset(data, conn) {
                // delete row about files from file_ref table
                // let count_del_files = delete_rows_by_uuids(files_of_set, conn);
                // debug!("Delete {:?} files of fileset {:?}", count_del_files, &data.fileset_uuid);

                return Ok(true)
            }
            Err(ServiceError::BadRequest(
                "Error with delete files of fileset data".to_string()
            ))
        },
    }
}

fn delete_file_row_of_fileset(
    data: &DelModificationFileFromFilesetData,
    conn: &PgConnection,
) -> bool {
    use crate::schema::modification_file_from_fileset::dsl::*;

    match diesel::delete(modification_file_from_fileset)
        .filter(fileset_uuid.eq(&data.fileset_uuid)
        .and(file_uuid.eq_any(&data.file_uuids)))
        .get_result::<ModificationFileFromFileset>(conn) {
        Ok(x) => {
            debug!("Completed delete files of fileset: {:?}", x);
            true
        },
        Err(err) => {
            debug!("Error delete files of fileset: {:?}", err);
            false
        }
    }
}
