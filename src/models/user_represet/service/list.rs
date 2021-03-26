use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::user_represet::model::UserRepreset;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    // debug!("fn show uuid_user_search = {}", &uuid_user_search);
    let uuid_user_search = Some(uuid_user_search);
    match uuid_user_search {
        Some(uuid_user_search) if uuid_user_search == Uuid::nil() => find_all_user_represets(context, limit, offset),
        Some(uuid_user_search) if uuid_user_search > Uuid::nil() => find_uuid_user_represets(context, uuid_user_search, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_user_represets(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<UserRepreset>(conn)?)
}

fn find_uuid_user_represets(
    context: &Context,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .filter(uuid_user.eq(uuid_user_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<UserRepreset>(conn)?)
}
