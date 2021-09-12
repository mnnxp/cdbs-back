use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::component_modification::file_to_set_modification::model::FileToSetModification;
use diesel::prelude::*;


pub(crate) fn get_files_set_modification(
    cxt: &Context<'_>,
    target_set_id: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<FileToSetModification>> {
    let mut variant_selection: u8 = 0;
    if target_set_id > 0 {
        variant_selection += 1;
    }

    match variant_selection {
        // 0 => find_all_set_files(cxt, limit, offset),
        1 => find_files_for_set_modification(cxt, target_set_id, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_files_for_set_modification(
    cxt: &Context<'_>,
    target_set_id: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<FileToSetModification>> {
    use crate::schema::file_to_set_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    Ok(file_to_set_modification
        .filter(set_id.eq(target_set_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<FileToSetModification>(conn)?)
}
