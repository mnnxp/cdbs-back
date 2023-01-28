use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::program::model::{
    Program, ProgramArg
};
use crate::schema::program_ref::dsl as program_ref;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_programs(
    args: &ProgramArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    match args.program_ids.is_empty() {
        true => find_all_program(&args.limit, &args.offset, conn),
        false => find_program_id(args, conn),
    }
}

fn find_all_program(
    limit: &i32,
    offset: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .limit(*limit as i64)
        .offset(*offset as i64)
        .order(program_ref::name.asc())
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_program_id(
    args: &ProgramArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .filter(program_ref::id.eq_any(&args.program_ids))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}
