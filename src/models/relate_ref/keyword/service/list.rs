use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::keyword::model::Keyword;
use diesel::prelude::*;


pub(crate) fn get_keywords(
    cxt: &Context<'_>,
    target_keyword_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    match target_keyword_id {
        target_keyword_id if target_keyword_id.is_empty() => find_all_keywords(cxt, limit, offset),
        target_keyword_id => find_keyword_ids(cxt, target_keyword_id, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_keywords(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    use crate::schema::keyword_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(keyword_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Keyword>(conn)?)
}

fn find_keyword_ids(
    cxt: &Context<'_>,
    target_keyword_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    use crate::schema::keyword_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(keyword_ref
        .filter(id.eq_any(target_keyword_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Keyword>(conn)?)
}
