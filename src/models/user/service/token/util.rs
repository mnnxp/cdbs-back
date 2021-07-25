use super::model::{UserToken, InsertableUserToken};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceError;
use crate::models::user::model::SlimUser;
use crate::models::user::service as user;
use crate::jwt::model::{Token, Claims};
use std::convert::TryFrom;
use async_graphql::Context;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// get token from request
pub(crate) fn token_from_context(context: &Context<'_>) -> Result<String, ServiceError> {
    // let token = context
    //     .data_opt::<String>()
    //     .map(|token| token.to_owned());

    match context.data_opt::<String>() {
        None => Err(ServiceError::Unauthorized),
        Some(token) => Ok(token.to_owned()),
    }
}

/// get SlimUser from Claims
pub(crate) fn get_slim_user(jwt: Claims) -> Result<SlimUser, ServiceError> {
    SlimUser::try_from(jwt).map_err(|e| ServiceError::BadRequest(e.to_string()))
}

/// update token for authorized user
pub(crate) fn update(context: &Context<'_>) -> Result<Token, ServiceError> {
        let conn: &PooledConnection = &get_conn(context)?;
        // get old token
        let old_token = user::token::token_from_context(context)?;
        println!("Token, old_token: {:?}", &old_token);
        // decrypt old token
        let old_data = user::token::decode(old_token.as_str())?;
        println!("Token, old_data: {:?}", &old_data);
        // deactivate old token
        let dis_token = disable_token(&old_token, conn)?;
        println!("Token, user: {:#?}", &dis_token);
        // get data from old token
        let user = user::token::get_slim_user(old_data)?;
        println!("Token, user: {:?}", &user);
        // creating a new token
        let new_token = user::token::generate(&user)?;
        println!("Token, new_token: {:?}", &new_token);
        // decrypt new token
        let new_data = user::token::decode(old_token.as_str())?;
        println!("Token, new_data: {:?}", &new_data);

        match new_token.bearer {
            None => Err(ServiceError::InternalServerError),
            Some(ref token) => {
                // insert data new token into the table
                println!("Token: {:#?}", token);
                let inserted_token = write_token(token, new_data, conn)?;
                println!("Inserted token: {:#?}", inserted_token);
                Ok(new_token)
            }
        }
}

/// disable token to table user_tokens_ref of database
fn disable_token(target_token: &str, conn: &PooledConnection) -> Result<UserToken, ServiceError> {
    use crate::schema::user_tokens_ref::dsl::*;

    let updated_token: UserToken = diesel::update(user_tokens_ref
        .filter(token.eq(&target_token)))
        .set(is_enabled.eq(false))
        .get_result(conn)?;
    Ok(updated_token)
}

/// write token to table user_tokens_ref of database
fn write_token(new_token: &str, jwt: Claims, conn: &PooledConnection) -> Result<UserToken, ServiceError> {
    use crate::schema::user_tokens_ref::dsl::user_tokens_ref;

    // creating a structure for writing token to a table
    let user_token = InsertableUserToken {
        uuid_user: Uuid::parse_str(&jwt.sub)?,
        token: new_token.to_string(),
        start_at: NaiveDateTime::from_timestamp(jwt.iat, 0),
        end_at: NaiveDateTime::from_timestamp(jwt.exp, 0),
        is_enabled: true,
    };

    let inserted_token: UserToken = diesel::insert_into(user_tokens_ref)
        .values(&user_token)
        .get_result(conn)?;
    Ok(inserted_token)
}
