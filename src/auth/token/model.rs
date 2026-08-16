use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::SlimUser;
use crate::schema::*;
use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use anyhow::Result;
use async_graphql::SimpleObject;
use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    // issuer
    pub(crate) iss: String,
    // subject - uuid user
    pub(crate) sub: Uuid,
    // issued at
    pub(crate) iat: i64,
    // expiry
    pub(crate) exp: i64,
    // user username
    pub(crate) username: String,
    // user default program
    pub(crate) program_id: i32,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(slim_user: &SlimUser, issuer: String, auth_duration_in_hour: u16) -> Self {
        let SlimUser {
            uuid,
            program_id,
            username,
            ..
        } = slim_user;

        let iat = Utc::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            iss: issuer,
            sub: *uuid,
            program_id: *program_id,
            username: username.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }

    pub(crate) fn days_until_expiry(&self) -> i64 {
        (self.exp - Utc::now().timestamp()) / 86400
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct Token {
    pub(crate) bearer: Option<String>,
}

impl Token {
    pub(crate) fn from_auth(token_str: &str) -> Self {
        Self {
            bearer: token_str
                .strip_prefix("Bearer ")
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .map(|t| t.to_string()),
        }
    }
}

impl TryFrom<Claims> for SlimUser {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims {
            program_id,
            username,
            sub,
            ..
        }: Claims = claims;

        Ok(SlimUser {
            uuid: sub,
            username,
            program_id,
        })
    }
}

/// get token from request
impl From<&HeaderMap> for Token {
    fn from(req: &HeaderMap) -> Self {
        let auth_op = req.get(AUTHORIZATION).and_then(|v| v.to_str().ok());
        auth_op.map(Self::from_auth).unwrap_or_default()
    }
}

/// User token data
#[derive(Debug, Queryable, SimpleObject)]
pub(crate) struct UserToken {
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// User Token (JWT)
    pub(crate) token: String,
    /// Token creation date
    pub(crate) created_at: NaiveDateTime,
    /// Token expiration date
    pub(crate) expiration_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_token_ref)]
pub(super) struct InsertableUserToken {
    user_uuid: Uuid,
    token: String,
    created_at: NaiveDateTime,
    expiration_at: NaiveDateTime,
}

impl InsertableUserToken {
    /// Create based on data without token
    pub(super) fn new(user_uuid: &Uuid, jwt: &Claims) -> ServiceResult<Self> {
        let created_at = NaiveDateTime::from_timestamp_opt(jwt.iat, 0)
            .ok_or(ServiceError::InternalServerError)?;
        let expiration_at = NaiveDateTime::from_timestamp_opt(jwt.exp, 0)
            .ok_or(ServiceError::InternalServerError)?;
        Ok(Self {
            user_uuid: *user_uuid,
            token: String::new(),
            created_at,
            expiration_at,
        })
    }

    /// Change token data
    pub(super) fn put_token(&mut self, token: &str) {
        self.token = token.to_string();
    }
}

impl From<UserToken> for InsertableUserToken {
    fn from(user_data: UserToken) -> Self {
        let UserToken {
            user_uuid,
            token,
            created_at,
            expiration_at,
            ..
        } = user_data;

        Self {
            user_uuid,
            token,
            created_at,
            expiration_at,
        }
    }
}
