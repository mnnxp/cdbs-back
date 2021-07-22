// use crate::cli_args::Opt;
use crate::errors::ServiceResult;
// use crate::jwt::model::{DecodedToken, Token};
use crate::models::user::model::ShowUser;
use crate::models::user::service as user;
use async_graphql::Context;
// use crate::database::PooledConnection;
// use diesel::PgConnection;

// use std::sync::Arc;
use uuid::Uuid;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by id
    async fn users(
        &self,
        context: &Context<'_>,
        uuid: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowUser>> {
        let uuid_user_create = match uuid {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        user::list::get_users(context, uuid_user_create, limit, offset)
    }
}
