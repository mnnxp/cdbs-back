use crate::errors::ServiceResult;
use crate::jwt::manager::create_token;
use crate::jwt::model::Token;
use crate::models::user::model::SlimUser;
use async_graphql::*;

#[Object]
impl Token {
    async fn bearer(&self) -> &Option<String> {
        &self.bearer
    }
}

pub(crate) fn generate(user: &SlimUser) -> ServiceResult<Token> {
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };
    match create_token(user, opt.auth_duration_in_hour) {
        Ok(r) => Ok(Token { bearer: Some(r) }),
        Err(e) => Err(e),
    }
}
