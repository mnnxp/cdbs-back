use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use async_graphql::Context;

/// Logout user and delete used token
pub(crate) fn logout_user(
    cxt: &Context<'_> ,
) -> Result<String, ServiceError> {
    use crate::models::user::access::token::{token_from_cxt, delete_token};

    let conn: &mut PooledConnection = &mut get_conn(cxt)?;

    let target_token = token_from_cxt(cxt)?;

    match delete_token(target_token.as_str(), conn) {
        Ok(_) => Ok("Good Luck".to_string()),
        Err(_) => Err(ServiceError::Unauthorized),
    }
}
