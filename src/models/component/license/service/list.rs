use crate::database::PooledConnection;
use crate::errors::{
    // ServiceError,
    ServiceResult
};
use crate::graphql::model::Context;
use crate::models::component::license::model::License;
use diesel::prelude::*;


pub(crate) fn get_licenses(
    context: &Context,
    id_license_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    match id_license_search {
        id_license_search if id_license_search.is_empty() => find_all_license(context, limit, offset),
        id_license_search => find_id_license(context, id_license_search, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_license(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    use crate::schema::license_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(license_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<License>(conn)?)
}

fn find_id_license(
    context: &Context,
    id_license_search: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<License>> {
    use crate::schema::license_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(license_ref
        .filter(id.eq_any(id_license_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<License>(conn)?)
}
