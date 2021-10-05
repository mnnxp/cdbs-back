use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::fileset_for_program::model::{
    FilesetProgram,
    DelFilesetProgramData,
};
use crate::models::component::component_modification::relate::fileset_for_program::util::get_component_by_fileset;
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete a set of files for the program
// not delete row in file_ref and file in storage
pub(crate) fn del_modification_fileset(
    logged_user_uuid: &Uuid, // <-- todo!(check access)
    data: &DelFilesetProgramData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::modification_file_from_fileset::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&data.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let find_files = &modification_file_from_fileset
        .filter(fileset_uuid.eq(&data.fileset_uuid))
        .select(file_uuid)
        .load::<Uuid>(conn);

    // debug!("Delete fileset: {:?}", del_fileset);

    match find_files {
        // just delete fileset when not have files
        Ok(files_of_set) if files_of_set.is_empty() => Ok(delete_fileset_row(data, conn)),
        // delete files of fileset, before delete fileset
        Ok(_) => {
            // delete row about fileset from fileset_for_program table
            if delete_fileset_row(data, conn) {
                // delete row about files from file_ref table
                // let count_del_files = delete_rows_by_uuids(files_of_set, conn);
                // debug!("Delete {:?} files of fileset {:?}", count_del_files, &data.fileset_uuid);

                return Ok(true)
            }
            Err(ServiceError::BadRequest(
                "Not found fileset data".to_string()
            ))
        },
        Err(err) => {
            debug!("Fail delete fileset: {:?}", err);
            Err(ServiceError::BadRequest(
                "Error with delete fileset data".to_string()
            ))
        }
    }
}

fn delete_fileset_row(
    data: &DelFilesetProgramData,
    conn: &PgConnection,
) -> bool {
    use crate::schema::fileset_for_program::dsl::*;

    match diesel::delete(fileset_for_program)
        .filter(uuid.eq(&data.fileset_uuid)
        .and(modification_uuid.eq(&data.modification_uuid)))
        .get_result::<FilesetProgram>(conn) {
        Ok(x) => {
            debug!("Completed delete fileset: {:?}", x);
            true
        },
        Err(err) => {
            debug!("Error delete fileset: {:?}", err);
            false
        }
    }
}
