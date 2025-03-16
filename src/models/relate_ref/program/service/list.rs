use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::program::model::Program;
use crate::models::search::order::Paginate;
use crate::schema::program_ref::dsl as program_ref;
use diesel::{PgConnection, prelude::*};

/// Returns a list of programs, filtered by IDs.
/// If no program filter is specified, all existing programs are aggregated.
pub(crate) fn get_programs(
    program_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    match program_ids.is_empty() {
        true => find_all_program(paginate, conn),
        false => find_program_id(program_ids, paginate, conn),
    }
}

fn find_all_program(
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .limit(paginate.limit)
        .offset(paginate.offset)
        .order(program_ref::name.asc())
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_program_id(
    program_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Program>> {
    program_ref::program_ref
        .filter(program_ref::id.eq_any(program_ids))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Program>(conn)
        .map_err(|err| {
            debug!("Failed get program: {:?}", err);
            ServiceError::InternalServerError
        })
}
