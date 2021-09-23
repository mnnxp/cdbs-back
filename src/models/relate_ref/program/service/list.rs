use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::program::model::Program;
use diesel::prelude::*;


pub(crate) fn get_programs(
    cxt: &Context<'_>,
    target_program_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    match target_program_id {
        target_program_id if target_program_id.is_empty() => find_all_program(cxt, limit, offset),
        target_program_id => find_program_id(cxt, target_program_id, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_program(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    use crate::schema::program_ref::dsl::*;

    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(program_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Program>(conn)?)
}

fn find_program_id(
    cxt: &Context<'_>,
    target_program_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Program>> {
    use crate::schema::program_ref::dsl::*;

    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(program_ref
        .filter(id.eq_any(target_program_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Program>(conn)?)
}
