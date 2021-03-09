use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::models::user::model::User;
use diesel::prelude::*;

pub(crate) fn find_all_users(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<User>> {
    use crate::schema::user_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<User>(conn)?)
}
