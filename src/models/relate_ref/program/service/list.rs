use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::program::model::Program;
use diesel::prelude::*;


pub(crate) fn get_programs(
    context: &Context<'_>,
    target_id_program: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    match target_id_program {
        target_id_program if target_id_program.is_empty() => find_all_program(context, limit, offset),
        target_id_program => find_id_program(context, target_id_program, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_program(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    use crate::schema::program_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(program_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Program>(conn)?)
}

fn find_id_program(
    context: &Context<'_>,
    target_id_program: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    use crate::schema::program_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(program_ref
        .filter(id.eq_any(target_id_program))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Program>(conn)?)
}
