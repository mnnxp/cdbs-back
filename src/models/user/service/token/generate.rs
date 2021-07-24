use crate::errors::ServiceResult;
// use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
// use async_graphql::Context;
use async_graphql::*;
use crate::models::user::model::SlimUser;
use crate::jwt::manager::create_token;
use crate::jwt::model::Token;

#[Object]
impl Token {
    async fn bearer(&self) -> &Option<String> {
        &self.bearer
    }
}

pub(crate) fn generate(user: &SlimUser) -> ServiceResult<Token> {
    // match context.user.0 {
    //     None => Err(ServiceError::Unauthorized),
    //     Some(ref user) => {
    //         match create_token(
    //             user,
    //             context.opt.domain.clone(),
    //             context.opt.auth_duration_in_hour,
    //         ) {
    //             Ok(r) => Ok(Token { bearer: Some(r) }),
    //             Err(e) => Err(e),
    //         }
    //     }
    // }
    // let user = context.data::<String>().map_err(|_| ServiceError::Unauthorized)?;
    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };
    match create_token(
        user,
        opt.domain.clone(),
        opt.auth_duration_in_hour,
    ) {
        Ok(r) => Ok(Token { bearer: Some(r) }),
        Err(e) => Err(e),
    }
}
