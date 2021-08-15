use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::actual_status::model::ActualStatus;
use diesel::prelude::*;


pub(crate) fn get_actual_status(
    context: &Context<'_>,
    target_id_actual_status: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ActualStatus>> {
    match target_id_actual_status {
        target_id_actual_status if target_id_actual_status.is_empty() => find_all_actual_status(context, limit, offset),
        target_id_actual_status => find_id_actual_status(context, target_id_actual_status, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_actual_status(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ActualStatus>> {
    use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;
    // let target_id_lang: IdLanguage = context.into();

    Ok(actual_status_ref
        // .filter(id_lang.eq_any(target_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ActualStatus>(conn)?)
}

fn find_id_actual_status(
    context: &Context<'_>,
    target_id_actual_status: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ActualStatus>> {
    use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(actual_status_ref
        // .filter(id_lang.eq_any(target_id_lang))
        .filter(id.eq_any(target_id_actual_status))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ActualStatus>(conn)?)
}
