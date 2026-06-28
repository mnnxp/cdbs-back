use crate::auth::token::{check_token, whose_token};
use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
use async_graphql::Context;
use uuid::Uuid;

use super::token::token_from_cxt;

/// Authorization context for authenticated requests
///
/// Provides convenient access to authenticated user data after successful validation from GraphQL context.
#[derive(Clone)]
pub(crate) struct AuthContext {
    pub(crate) user_uuid: Uuid,
    // pub(crate) token: String,
}

impl AuthContext {
    /// Creates an authorized context from GraphQL request context
    ///
    /// # Errors
    /// Returns:
    /// - `ErrorMessage::TokenNotFound` if token is missing or invalid
    /// - `ErrorMessage::TokenExpired` if token is expired
    /// - `ErrorMessage::TokenIsInvalid` if failed decode the token
    /// - `ServiceError::Unauthorized` if token is revoked or expired in database
    ///
    /// # Database errors
    /// Other database errors are propagated as their respective `ServiceError` variants
    pub(crate) fn from_graphql(cxt: &Context<'_>) -> ServiceResult<Self> {
        let token = token_from_cxt(cxt)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        if !check_token(&token, conn)? {
            return Err(ServiceError::Unauthorized);
        }
        let user_uuid = whose_token(&token, conn)?;
        Ok(Self { user_uuid })
    }

    /// Returns the authenticated user's UUID
    pub(crate) fn user_uuid(&self) -> Uuid {
        self.user_uuid
    }

    // /// Returns the JWT token string used for authentication
    // pub(crate) fn token(&self) -> &str {
    //     &self.token
    // }
}
