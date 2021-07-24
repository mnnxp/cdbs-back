use crate::errors::ServiceError;
use crate::models::user::model::SlimUser;
use crate::jwt::model::Claims;
use std::convert::TryFrom;
use async_graphql::Context;

/// get token from request
pub(crate) fn token_from_context<'a>(context: &Context<'a>) -> Result<&'a String, ServiceError> {
    // let token = context
    //     .data_opt::<String>()
    //     .map(|token| token.to_owned());

    match context.data_opt::<String>() {
        None => Err(ServiceError::Unauthorized),
        Some(token) => Ok(token),
    }
}

/// get SlimUser from Claims
pub(crate) fn get_slim_user(jwt: Claims) -> Result<SlimUser, ServiceError> {
    SlimUser::try_from(jwt).map_err(|_| ServiceError::InternalServerError)
}
