use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::language::model::Language;
use diesel::prelude::*;


pub(crate) fn get_languages(
    cxt: &Context<'_>,
    target_lang_iduage: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    match target_lang_iduage {
        target_lang_iduage if target_lang_iduage.is_empty() => find_all_languages(cxt, limit, offset),
        target_lang_iduage => find_lang_iduage(cxt, target_lang_iduage, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_languages(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    use crate::schema::language_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(language_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Language>(conn)?)
}

fn find_lang_iduage(
    cxt: &Context<'_>,
    target_lang_iduage: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Language>> {
    use crate::schema::language_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(language_ref
        .filter(id.eq_any(target_lang_iduage))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Language>(conn)?)
}
