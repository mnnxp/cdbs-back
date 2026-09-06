use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use async_graphql::Context;

/// Logout user and delete used token
pub(crate) fn logout_user(ctx: &Context<'_>) -> Result<String, ServiceError> {
    use crate::auth::token::{delete_token, token_from_ctx};

    let conn: &mut PooledConnection = &mut get_conn(ctx)?;

    let target_token = token_from_ctx(ctx)?;

    match delete_token(target_token.as_str(), conn) {
        Ok(_) => Ok("Good Luck".to_string()),
        Err(_) => Err(ServiceError::Unauthorized),
    }
}
