use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::user_represet::model::UserRepreset;
use diesel::prelude::*;
use std::any::Any;


pub(crate) fn show(
    context: &Context,
    id_serch: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    let id_serch = Some(id_serch);
    match id_serch {
        Some(id_serch) if id_serch == 0 => find_all_user_represets(context, limit, offset),
        Some(id_serch) => find_id_user_represets(context, id_serch, limit, offset),
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

fn find_id_user_represets(
    context: &Context,
    id_serch: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<UserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .filter(id_user.eq(id_serch))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<UserRepreset>(conn)?)
}
