use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::fileset_for_program::model::{
    FilesetProgram,
    IptFilesetProgramData,
    InsertableFilesetProgram
};
use diesel::prelude::*;
use uuid::Uuid;

/// Creating a new set of files for the program
/// if found duplicate (modification and program) return error with fileset_uuid
pub(crate) fn create_modification_fileset(
    data: IptFilesetProgramData,
    conn: &PgConnection
) -> ServiceResult<FilesetProgram> {
    use crate::schema::fileset_for_program::dsl::*;

    let find_fileset = &fileset_for_program
        .filter(modification_uuid.eq(&data.modification_uuid)
        .and(program_id.eq(&data.program_id)))
        .select(uuid)
        .limit(1)
        .load::<Uuid>(conn);

    // debug!("Find fileset: {:?}", find_fileset);

    let find_fileset = match find_fileset {
        Ok(set) => &*set,
        Err(err) => {
            debug!("Not found target fileset_for_program: {:?}", err);
            return Err(ServiceError::BadRequest(
                "Error with connect database".to_string()
            ))
        }
    };

    match find_fileset.get(0).take() {
        Some(x) => Err(ServiceError::BadRequest(
            format!("The modification has a set of files for this program: {:?}", x)
        )),
        None => {
            let data: InsertableFilesetProgram = data.into();

            let inserted_param_data: FilesetProgram = diesel::insert_into(fileset_for_program)
                .values(&data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
    }
}
