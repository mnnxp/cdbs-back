use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::program::model::{InsertableProgram, IptProgramData, Program};
use crate::schema::program_ref::dsl as program_ref;
use diesel::prelude::*;
// use uuid::Uuid;

/// Добавляет новое имя программы.
/// Возвращает идентификатор и наименование программы.
pub(crate) fn create_program(
    new_program_data: &IptProgramData,
    conn: &mut PgConnection,
) -> ServiceResult<Program> {
    let flag_found = program_ref::program_ref
        .filter(program_ref::name.eq(&new_program_data.name))
        .select(program_ref::id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check program: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.first() {
        Some(x) => Err(get_err_msg(ErrorMessage::NameAlreadyThereX(
            "program".to_string(),
            *x,
        ))),
        None => {
            let new_program_data: InsertableProgram = new_program_data.into();
            diesel::insert_into(program_ref::program_ref)
                .values(&new_program_data)
                .get_result::<Program>(conn)
                .map_err(|err| {
                    debug!("Failed insert program: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}
