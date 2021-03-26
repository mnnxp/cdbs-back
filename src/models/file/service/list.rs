use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::models::file::model::File;
use diesel::prelude::*;
use crate::schema::file_ref::dsl::file_ref;

pub(crate) fn find_all_files(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<File>> {
    // use crate::schema::user_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(file_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<File>(conn)?)
}
