use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::language::model::Language;
use diesel::prelude::*;


pub(crate) fn get_languages(
    context: &Context<'_>,
    target_id_language: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    match target_id_language {
        target_id_language if target_id_language.is_empty() => find_all_languages(context, limit, offset),
        target_id_language => find_id_language(context, target_id_language, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_languages(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    use crate::schema::language_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(language_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Language>(conn)?)
}

fn find_id_language(
    context: &Context<'_>,
    target_id_language: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    use crate::schema::language_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(language_ref
        .filter(id.eq_any(target_id_language))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Language>(conn)?)
}
