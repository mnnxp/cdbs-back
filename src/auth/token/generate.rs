use crate::auth::token::manager::create_token;
use crate::auth::token::model::Token;
use crate::config::auth_duration_in_hour;
use crate::errors::ServiceResult;
use crate::models::user::model::SlimUser;
use async_graphql::*;

#[Object]
impl Token {
    async fn bearer(&self) -> &Option<String> {
        &self.bearer
    }
}

pub(crate) fn generate(user: &SlimUser) -> ServiceResult<Token> {
    let duration = auth_duration_in_hour();
    match create_token(user, duration) {
        Ok(r) => Ok(Token { bearer: Some(r) }),
        Err(e) => Err(e),
    }
}
