use super::{check_token, token_from_cxt, whose_token};
use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
use async_graphql::Context;
use uuid::Uuid;

/// Checking user authorization
/// Return error if token not found or not valid
pub(crate) fn check_authorized(cxt: &Context<'_>) -> ServiceResult<bool> {
    let conn: &mut PooledConnection = &mut get_conn(cxt)?;
    let token = token_from_cxt(cxt)?;
    match check_token(token.as_str(), conn)? {
        true => Ok(true),
        false => Err(ServiceError::Unauthorized),
    }
}

/// Retrieves the UUID of the authenticated user from the GraphQL context.
/// Send query to database token validation.
pub(crate) fn get_logged_user_uuid(cxt: &Context<'_>, need_check: bool) -> ServiceResult<Uuid> {
    let target_token = token_from_cxt(cxt)?;
    let conn: &mut PooledConnection = &mut get_conn(cxt)?;
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