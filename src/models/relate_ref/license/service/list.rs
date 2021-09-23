use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::license::model::License;
use diesel::prelude::*;


pub(crate) fn get_licenses(
    cxt: &Context<'_>,
    license_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    match license_id_search {
        license_id_search if license_id_search.is_empty() => find_all_license(cxt, limit, offset),
        license_id_search => find_license_id(cxt, license_id_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_license(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    use crate::schema::license_ref::dsl::*;

    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(license_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<License>(conn)?)
}

fn find_license_id(
    cxt: &Context<'_>,
    license_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    use crate::schema::license_ref::dsl::*;

    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(license_ref
        .filter(id.eq_any(license_id_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<License>(conn)?)
}
