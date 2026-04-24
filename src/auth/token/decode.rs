use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use crate::auth::jwt::manager::decode_token;
use crate::auth::jwt::model::Claims;

use async_graphql::*;
use uuid::Uuid;

// pub(crate) type ClaimsResponse = Claims;

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
    async fn iat(&self) -> String {
        chrono::NaiveDateTime::from_timestamp_opt(self.iat, 0)
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
    }
    async fn exp(&self) -> String {
        chrono::NaiveDateTime::from_timestamp_opt(self.exp, 0)
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
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
