use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::component_modification::file_to_set_modification::model::FileToSetModification;
use async_graphql::Context;
use diesel::prelude::*;

pub(crate) fn get_files_set_modification(
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
