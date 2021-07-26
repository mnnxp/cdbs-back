use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use crate::models::user::service as user;
use async_graphql::Context;

pub(crate) fn logout(
    context: &Context<'_> ,
) -> Result<String, ServiceError> {
    let conn: &PooledConnection = &get_conn(&context)?;

    let target_token = user::token::token_from_context(&context)?;

    match user::token::delete_token(target_token.as_str(), conn) {
        Ok(_) => Ok("Good Luck".to_string()),
        Err(_) => Err(ServiceError::Unauthorized),
    }
}
