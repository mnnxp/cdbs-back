use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::extension::model::Extension;
use diesel::prelude::*;


pub(crate) fn get_extensions(
    context: &Context<'_>,
    target_id_extension: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Extension>> {
    match target_id_extension {
        target_id_extension if target_id_extension.is_empty() => find_all_extension(context, limit, offset),
        target_id_extension => find_id_extension(context, target_id_extension, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_extension(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Extension>> {
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(extension_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Extension>(conn)?)
}

fn find_id_extension(
    context: &Context<'_>,
    target_id_extension: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Extension>> {
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(extension_ref
        .filter(id.eq_any(target_id_extension))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Extension>(conn)?)
}
