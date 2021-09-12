use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::program::model::{InsertableProgram, Program, IptProgramData};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_program(
    new_program_data: IptProgramData,
    conn: &PgConnection
) -> ServiceResult<Program> {
    use crate::schema::program_ref::dsl::*;
    // use crate::schema::program_to_component::dsl::uuid as component_uuid;
    // use crate::schema::program_to_modification::dsl::uuid as modification_uuid;
    // use diesel::dsl::count;

    let flag_found_program = program_ref
        .filter(name.eq(&new_program_data.name))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_program START SEARCH ={:?}", flag_found_program);

    match flag_found_program {
        0 => {
            let new_program_data: InsertableProgram = new_program_data.into();
            let inserted_program_data: Program = diesel::insert_into(program_ref)
                .values(&new_program_data)
                .get_result(conn)?;
            Ok(inserted_program_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This program name is already there. Id: {}", flag_found_program))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
