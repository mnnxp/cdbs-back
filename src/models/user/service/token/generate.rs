use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::user::model::SlimUser;
use crate::jwt::manager::create_token;
use crate::jwt::model::Token;

pub(crate) fn generate<'a>(context: &'a Context) -> ServiceResult<Token> {
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
    let user = context.data::<SlimUser>().map_err(|_| ServiceError::Unauthorized)?;
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
