use crate::auth::middleware::AuthStatus;
use crate::auth::token::{find_user_by_token, token_from_ctx};
use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
use async_graphql::Context;
use uuid::Uuid;

/// Checking user authorization
/// Return error if token not found or not valid
pub(crate) fn check_authorized(ctx: &Context<'_>) -> ServiceResult<bool> {
    get_logged_user_uuid(ctx).map(|user_uuid| !user_uuid.is_nil())
}

/// Retrieves the UUID of the authenticated user from the GraphQL context.
/// Send query to database token validation.
pub(crate) fn get_logged_user_uuid(ctx: &Context<'_>) -> ServiceResult<Uuid> {
    // Check auth context from middleware first
    if let Some(auth_ctx) = ctx.data_opt::<AuthStatus>().as_ref() {
        match auth_ctx {
            AuthStatus::Authenticated(user_uuid) => Ok(*user_uuid),
            AuthStatus::Invalid(service_error) => Err(service_error.clone()),
            AuthStatus::Anonymous => Err(ServiceError::Unauthorized),
        }
    } else {
        debug!("Fail get logged user uuid");
        let target_token = token_from_ctx(ctx)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        find_user_by_token(target_token.as_str(), conn)
    }
}
