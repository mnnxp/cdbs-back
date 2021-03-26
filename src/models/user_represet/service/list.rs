use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::user_represet::model::UserRepreset;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_serch: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    let uuid_serch = Some(uuid_serch);
    match uuid_serch {
        Some(uuid_serch) if uuid_serch == Uuid::nil() => find_all_user_represets(context, limit, offset),
        Some(uuid_serch) if uuid_serch > Uuid::nil() => find_uuid_user_represets(context, uuid_serch, limit, offset),
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
    uuid_serch: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .filter(uuid_user.eq(uuid_serch))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<UserRepreset>(conn)?)
}
