use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use crate::jwt::model::Claims;
use crate::jwt::manager::decode_token;

use async_graphql::*;

// pub type ClaimsResponse = Claims;

#[Object]
impl Claims {
    async fn iss(&self) -> &String {
        &self.iss
    }
    async fn username(&self) -> &String {
        &self.username
    }
    async fn sub(&self) -> &String {
        &self.sub
    }
    async fn iat(&self) -> String {
        chrono::NaiveDateTime::from_timestamp(self.iat, 0)
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
    }
    async fn exp(&self) -> String {
        chrono::NaiveDateTime::from_timestamp(self.exp, 0)
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
    }
}

pub(crate) fn decode(token: &str) -> ServiceResult<Claims>  {
    decode_token(token).map_err(|e| ServiceError::BadRequest(e.to_string()))
    // match context.token.jwt {
    //     None => Err(ServiceError::Unauthorized),
    //     Some(ref m) => Ok(m as &ClaimsResponse),
    // }
    // match token.bearer {
    //     None => Err(ServiceError::Unauthorized),
    //     Some(ref bearer) => {
    //         let jwt = decode_token(bearer).map_err(|_| ServiceError::InternalServerError)?;
    //         Ok(jwt)
    //     },
    // }
    // let jwt = context.data::<Claims>().map_err(|_| ServiceError::Unauthorized)?;
    // Ok(jwt as &ClaimsResponse)
}
