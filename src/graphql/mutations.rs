use async_graphql::{Context, Result};

// use crate::database::PooledConnection;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{SlimUser, UserData, IptUserData};
use crate::models::user::service as user;
pub struct MutationRoot;
// use diesel::PgConnection;
use diesel::pg::PgConnection;

use crate::database::Pool;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        context: &Context<'_>,
        data: IptUserData,
    ) -> ServiceResult<SlimUser> {
        let conn = context
            .data::<Pool>()
            .expect("Can't get pool")
            .get()
            .expect("Can't get DB connection");
        user::register::create_user(data.into(), &conn)
    }
}
