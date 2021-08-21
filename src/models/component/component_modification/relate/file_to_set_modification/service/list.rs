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
    context: &Context<'_>,
    target_id_set: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<FileToSetModification>> {
    let mut variant_selection: u8 = 0;
    if target_id_set > 0 {
        variant_selection += 1;
    }

    match variant_selection {
        // 0 => find_all_set_files(context, limit, offset),
        1 => find_files_for_set_modification(context, target_id_set, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_files_for_set_modification(
    context: &Context<'_>,
    target_id_set: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<FileToSetModification>> {
    use crate::schema::file_to_set_modification::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(file_to_set_modification
        .filter(id_set.eq(target_id_set))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<FileToSetModification>(conn)?)
}
