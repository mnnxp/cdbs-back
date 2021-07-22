use async_graphql::Context;

// use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::models::user::model::{SlimUser, UserData};
use crate::models::user::service as user;
pub struct MutationRoot;
use diesel::PgConnection;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        context: &Context<'_>,
        data: UserData,
    ) -> ServiceResult<SlimUser> {
        let conn: &PgConnection = &context.db;

        user::register::create_user(data, conn).await
    }
}
