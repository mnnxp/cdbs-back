use actix_web::dev::{Service, ServiceRequest, Transform};
use actix_web::http::header;
use actix_web::{web, HttpMessage};
use futures::future::{ready, Ready};
use log::debug;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use uuid::Uuid;

use super::api_key::repository::validate_api_key;
use super::token::find_user_by_token;
use super::token::model::Token;
use crate::database::PgPool;
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceError;

/// Authentication context that can be stored in request extensions
#[derive(Debug, Clone)]
pub(crate) enum AuthStatus {
    /// Authenticated user with valid UUID
    Authenticated(Uuid),
    /// No authentication credentials provided (anonymous)
    Anonymous,
    /// Invalid token or API key provided
    Invalid(ServiceError),
}

/// Authentication middleware for Actix-web.
///
/// Validates requests by checking either:
/// - `X-API-Key` header (API keys for machine-to-machine access)
/// - `Authorization: Bearer <jwt>` header (JWT tokens for user sessions)
///
/// # Errors
///
/// The middleware returns `ServiceError::Unauthorized` in the following cases:
/// - **TokenNotFound**: No `Authorization` or `X-API-Key` header present (treated as anonymous)
/// - **TokenIsInvalid**: Failed to decode or verify JWT token
/// - **InvalidApiKey**: API key not found in database, expired, or inactive
///
/// # Database errors
///
/// Database errors are logged and forwarded as `ServiceError::InternalServerError`.
///
/// # Anonymous access
///
/// If no authentication credentials are provided, the request proceeds as anonymous.
pub(crate) struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest> + 'static,
    S::Error: From<ServiceError>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub(crate) struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest> + 'static,
    S::Error: From<ServiceError>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = match req.app_data::<web::Data<PgPool>>() {
            Some(p) => p.clone(),
            None => {
                return Box::pin(async move { Err(ServiceError::InternalServerError.into()) });
            }
        };

        let api_key = req
            .headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        let auth_str = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // TokenNotFound status if neither the token nor the key is present
            if auth_str.is_none() && api_key.is_none() {
                let status = AuthStatus::Invalid(get_err_msg(ErrorMessage::TokenNotFound));
                req.extensions_mut().insert(status);
                return service.call(req).await;
            }

            let auth_status = web::block(move || {
                let mut conn = pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?;

                // 1. Check API key
                if let Some(key_str) = api_key {
                    return match validate_api_key(&key_str, &mut conn) {
                        Ok(user_uuid) => Ok(AuthStatus::Authenticated(user_uuid)),
                        Err(err) => Ok(AuthStatus::Invalid(err)),
                    };
                }

                // 2. Check JWT
                let auth_str = match auth_str {
                    Some(s) => s,
                    None => return Ok(AuthStatus::Anonymous),
                };

                let status = Token::from_auth(&auth_str)
                    .bearer
                    .as_deref()
                    .map(|token| match find_user_by_token(token, &mut conn) {
                        Ok(user_uuid) => AuthStatus::Authenticated(user_uuid),
                        Err(err) => AuthStatus::Invalid(err),
                    })
                    .unwrap_or_else(|| {
                        AuthStatus::Invalid(get_err_msg(ErrorMessage::TokenIsInvalid))
                    });

                debug!("Token provided: {:?}", status);
                Ok::<_, ServiceError>(status)
            })
            .await
            .map_err(|_| ServiceError::InternalServerError)?
            .map_err(S::Error::from)?;

            req.extensions_mut().insert(auth_status);
            service.call(req).await
        })
    }
}
