use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::program::model::Program;
use crate::schema::program_ref::dsl as program_ref;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_programs(
    target_program_id: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Program>> {
    match target_program_id.is_empty() {
        true => find_all_program(limit, offset, conn),
        false => find_program_id(target_program_id, limit, offset, conn),
    }
}

fn find_all_program(
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_program_id(
    target_program_id: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .filter(program_ref::id.eq_any(target_program_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}
