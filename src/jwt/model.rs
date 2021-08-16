use crate::models::user::model::SlimUser;
use anyhow::Result;
use chrono::{Duration, Local};
use std::convert::TryFrom;
use uuid::Uuid;
use actix_web::{http::header, http::HeaderMap};
use regex::Regex;

lazy_static::lazy_static! {
    static ref BEARER_REGEXP : Regex = Regex::new(r"^Bearer\s(.*)$").expect("Bearer regexp failed!");
}

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject - uuid user
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user username
    pub username: String,
    // user is supplier
    pub id_program: i32,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(slim_user: &SlimUser, issuer: String, auth_duration_in_hour: u16) -> Self {
        let SlimUser {
            uuid,
            id_program,
            username,
            ..
        } = slim_user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            iss: issuer,
            sub: uuid.to_string(),
            id_program: *id_program,
            username: username.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub bearer: Option<String>,
}

impl TryFrom<Claims> for SlimUser {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims {
            id_program, username, sub, ..
        }: Claims = claims;

        Ok(SlimUser {
            uuid: Uuid::parse_str(&sub)?,
            username,
            id_program,
        })
    }
}

/// get token from request
impl From<&HeaderMap> for Token {
    fn from(req: &HeaderMap) -> Self {
        let token = req
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|authorization| {
                BEARER_REGEXP
                    .captures(authorization)
                    .and_then(|captures| captures.get(1))
            })
            .map(|v| v.as_str());

        Self {
            bearer: token.map(|t| t.to_string())
        }
    }
}
