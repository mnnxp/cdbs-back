use crate::errors::ServiceResult;
use async_graphql::Context;
use uuid::Uuid;

use crate::auth::token::logged::get_logged_user_uuid;

/// Authorization context for authenticated requests.
/// Provides convenient access to authenticated user data after successful validation from GraphQL context.
#[derive(Clone, Copy)]
pub(crate) struct AuthContext {
    pub(crate) user_uuid: Uuid,
}

impl AuthContext {
    /// Creates an authorized context from GraphQL request context
    ///
    /// # Errors
    /// Returns:
    /// - `ErrorMessage::TokenNotFound` if token is missing or invalid
    /// - `ErrorMessage::TokenIsInvalid` if failed decode the token
    /// - `ServiceError::Unauthorized` if token is expired or revoked in database
    ///
    /// # Database errors
    /// Other database errors are propagated as their respective `ServiceError` variants
    pub(crate) fn from_graphql(ctx: &Context<'_>) -> ServiceResult<Self> {
        let user_uuid = get_logged_user_uuid(ctx)?;
        debug!("AuthContext user_uuid: {:?}", user_uuid);
        Ok(Self { user_uuid })
    }

    /// Returns the authenticated user's UUID
    pub(crate) fn user_uuid(&self) -> Uuid {
        self.user_uuid
    }
}
