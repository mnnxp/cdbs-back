use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::set_of_files_program::model::{
    SetOfFilesProgram,
    IptSetOfFilesProgramData,
    InsertableSetOfFilesProgram
};
use diesel::prelude::*;

pub(crate) fn create_set_file_modification(
    new_param_data: IptSetOfFilesProgramData,
    conn: &PgConnection
) -> ServiceResult<SetOfFilesProgram> {
    use crate::schema::set_files_for_program::dsl::*;
    let new_param_data: InsertableSetOfFilesProgram = new_param_data.into();

    let flag_found_param = set_files_for_program
        .filter(modification_uuid.eq(&new_param_data.modification_uuid))
        .filter(program_id.eq(&new_param_data.program_id))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param as i32 {
        0 => {
            let inserted_param_data: SetOfFilesProgram = diesel::insert_into(set_files_for_program)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already with the modification.".to_string())),
    }
}
