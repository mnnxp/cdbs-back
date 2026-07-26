use crate::auth::token::manager::decode_token;
use crate::auth::token::model::{Claims, Token};
use crate::auth::token::{
    decode, delete_all_tokens, delete_user_token, show_tokens, token_from_cxt, update,
};
use crate::auth::token::{find_user_by_token, UserToken};
use crate::errors::{ServiceError, ServiceResult};
use async_graphql::Context;
use diesel::prelude::PgConnection;
use uuid::Uuid;

/// Returns active tokens of the authenticated user.
pub(crate) fn show_user_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserToken>> {
    show_tokens(logged_user_uuid, conn)
}

/// Returns true if current token is still valid (not expired or revoked)
pub(crate) fn check_token_valid(cxt: &Context<'_>, conn: &mut PgConnection) -> ServiceResult<bool> {
    let token_str = token_from_cxt(cxt)?;
    match find_user_by_token(&token_str, conn) {
        Ok(_user_uuid) => Ok(true),
        Err(ServiceError::Unauthorized) => Ok(false),
        Err(err) => Err(err),
    }
}

/// Returns days until token expiration for current user
pub(crate) fn get_token_days_until_expiry(cxt: &Context<'_>) -> ServiceResult<i64> {
    let token_str = token_from_cxt(cxt)?;
    let claims = decode_token(&token_str)?;
    Ok(claims.days_until_expiry())
}

/// Generates a new token without deleting other active tokens.
/// Returns the user's new authorization token.
pub(crate) fn get_user_token(cxt: &Context<'_>) -> ServiceResult<Token> {
    update(cxt, false)
}

/// Generates a new token and deactivates the current token.
/// Returns the user's new authorization token.
pub(crate) fn update_user_token(cxt: &Context<'_>) -> ServiceResult<Token> {
    update(cxt, true)
}

/// Returns token provider, user UUID, username, program ID, issued at, and expiration date.
pub(crate) fn decode_user_token(cxt: &Context<'_>) -> ServiceResult<Claims> {
    decode(&token_from_cxt(cxt)?)
}

/// Deactivates the specified user token.
pub(crate) fn delete_target_token(
    logged_user_uuid: &Uuid,
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    delete_user_token(logged_user_uuid, target_token, conn)
}

/// Deactivates all tokens of the user.
pub(crate) fn delete_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    delete_all_tokens(logged_user_uuid, conn)
}
