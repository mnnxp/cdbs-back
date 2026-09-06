use crate::auth::token::manager::decode_token;
use crate::auth::token::model::Claims;
use crate::errors::{ServiceError, ServiceResult};
use async_graphql::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[Object]
impl Claims {
    async fn iss(&self) -> &String {
        &self.iss
    }

    async fn username(&self) -> &String {
        &self.username
    }

    async fn sub(&self) -> &Uuid {
        &self.sub
    }

    async fn iat(&self) -> ServiceResult<String> {
        NaiveDateTime::from_timestamp_opt(self.iat, 0)
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.f").to_string())
            .ok_or(ServiceError::InternalServerError)
    }

    async fn exp(&self) -> ServiceResult<String> {
        NaiveDateTime::from_timestamp_opt(self.exp, 0)
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.f").to_string())
            .ok_or(ServiceError::InternalServerError)
    }

    async fn program_id(&self) -> i32 {
        self.program_id
    }
}

pub(crate) fn decode(token: &str) -> ServiceResult<Claims> {
    decode_token(token).map_err(|e| {
        debug!("Err decode: {:?}", e);
        ServiceError::InternalServerError
    })
}
