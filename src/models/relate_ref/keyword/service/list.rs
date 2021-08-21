use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::keyword::model::Keyword;
use diesel::prelude::*;


pub(crate) fn get_keywords(
    context: &Context<'_>,
    target_id_keyword: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    match target_id_keyword {
        target_id_keyword if target_id_keyword.is_empty() => find_all_keywords(context, limit, offset),
        target_id_keyword => find_id_keywords(context, target_id_keyword, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_keywords(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    use crate::schema::keyword_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(keyword_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Keyword>(conn)?)
}

fn find_id_keywords(
    context: &Context<'_>,
    target_id_keyword: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    use crate::schema::keyword_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(keyword_ref
        .filter(id.eq_any(target_id_keyword))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Keyword>(conn)?)
}
