use crate::errors::{ServiceResult, ServiceError};
use crate::database::{get_conn, PooledConnection};

use async_graphql::Context;
use uuid::Uuid;

/// Checking user authorization
/// Return error if token not found or not valid
pub(crate) fn check_authorized(
    cxt: &Context<'_>
) -> ServiceResult<bool> {
    use crate::models::user::access::token::{token_from_cxt, check_token};

    let conn: &PooledConnection = &get_conn(cxt)?;

    let token = token_from_cxt(cxt)?;

    match check_token(token.as_str(), conn)? {
        true => Ok(true),
        false => Err(ServiceError::Unauthorized),
    }
}

/// get user_uuid of the authorized user
/// with and without checking valid token
pub(crate) fn get_logged_user_uuid(
    cxt: &Context<'_>,
    need_check: bool
) -> ServiceResult<Uuid> {
    use crate::models::user::access::token::{token_from_cxt, whose_token, check_token};

    let target_token = token_from_cxt(cxt)?;

    let conn: &PooledConnection = &get_conn(cxt)?;

    match need_check {
        false => whose_token(target_token.as_str(), conn),
        true => {
            if check_token(target_token.as_str(), conn)? {
                whose_token(target_token.as_str(), conn)
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
    }
}
