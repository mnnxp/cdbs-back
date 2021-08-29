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
    let token = match context.data_opt::<Token>() {
        Some(token) => token.clone(),
        None => Token { bearer: None },
    };
    match token.bearer {
        Some(bearer) => Ok(bearer),
        None => Err(ServiceError::BadRequest("Token not found.".to_string())),
    }
}

/// show all tokens for uuid_user
pub(crate) fn show_tokens(
    context: &Context<'_>,
    auth_uuid_user: Uuid,
) -> Result<Vec<UserToken>, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;
    use crate::schema::user_token_ref::dsl::*;

    user_token_ref
        .filter(uuid_user.eq(auth_uuid_user))
        .load(conn)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

/// get SlimUser from Claims
pub(crate) fn get_slim_user(jwt: Claims) -> Result<SlimUser, ServiceError> {
    SlimUser::try_from(jwt)
        .map_err(|_| ServiceError::BadRequest("Fail get SlimUser from Claims.".to_string()))
}

/// updating a token with or without removing the old one
pub(crate) fn update(context: &Context<'_>, flag_delete_token: bool) -> Result<Token, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;
    // get old token
    let old_token = user::token::token_from_context(context)?;
    if check_token(old_token.as_str(), conn)? {
        // decrypt old token
        let old_data = user::token::decode(old_token.as_str())?;
        if flag_delete_token {
            // deactivate old token
            delete_token(old_token.as_str(), conn)?;
        }
        // get data from old token
        let user = user::token::get_slim_user(old_data)?;
        // creating a new token
        let new_token = user::token::generate(&user)?;
        // decrypt new token
        let new_data = user::token::decode(old_token.as_str())?;

        match new_token.bearer {
            None => Err(ServiceError::InternalServerError),
            Some(ref token) => {
                // insert data new token into the table
                write_token(token, new_data, conn)?;
                Ok(new_token)
            }
        }
    } else {
        Err(ServiceError::BadRequest("Your token is invalid.".to_string()))
    }
}

/// delete token to table user_token_ref of database
pub(crate) fn delete_token(target_token: &str, conn: &PooledConnection) -> Result<UserToken, ServiceError> {
    use crate::schema::user_token_ref::dsl::*;

    let updated_token: UserToken = diesel::delete(user_token_ref)
        .filter(token.eq(&target_token))
        .get_result(conn)?;
    Ok(updated_token)
}

/// delete target token to table user_token_ref of database
pub(crate) fn delete_user_token(
    context: &Context<'_>,
    target_token: &str,
    auth_uuid_user: Uuid,
) -> Result<i32, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;
    use crate::schema::user_token_ref::dsl::*;

    let updated_token: usize = diesel::delete(user_token_ref)
        .filter(uuid_user.eq(&auth_uuid_user))
        .filter(token.eq(&target_token))
        .execute(conn)?;
    Ok(updated_token as i32)
}

/// delete tokens to table user_token_ref of database
pub(crate) fn delete_all_tokens(
    context: &Context<'_>,
    target_uuid_user: Uuid,
) -> Result<i32, ServiceError> {
    let conn: &PooledConnection = &get_conn(context)?;
    use crate::schema::user_token_ref::dsl::*;

    let updated_token: usize = diesel::delete(user_token_ref)
        .filter(uuid_user.eq_all(&target_uuid_user))
        .execute(conn)?;
    Ok(updated_token as i32)
}

/// write token to table user_token_ref of database
pub(crate) fn write_token(
    new_token: &str,
    jwt: Claims,
    conn: &PooledConnection,
) -> Result<UserToken, ServiceError> {
    use crate::schema::user_token_ref::dsl::user_token_ref;
    use crate::schema::user_token_ref::dsl::token;

    // find duplicate token
    let find_token = user_token_ref
        .filter(token.eq(new_token))
        .execute(conn).unwrap_or(0);

    // check for no duplicate token
    match find_token {
        0 => {
            // creating a structure for writing token to a table
            let user_token = InsertableUserToken {
                uuid_user: Uuid::parse_str(&jwt.sub)?,
                token: new_token.to_string(),
                created_at: NaiveDateTime::from_timestamp(jwt.iat, 0),
                expiration_at: NaiveDateTime::from_timestamp(jwt.exp, 0),
            };

            let inserted_token: UserToken = diesel::insert_into(user_token_ref)
            .values(&user_token)
            .get_result(conn)?;
            Ok(inserted_token)
        },
        1 => Err(ServiceError::BadRequest("Please, try again later.".to_string())),
        _ => Err(ServiceError::BadRequest("Duplicate token found.".to_string())),
    }
}

/// check token for validity
pub(crate) fn check_token(
    target_token: &str,
    conn: &PooledConnection,
) -> Result<bool, ServiceError> {
    use crate::schema::user_token_ref::dsl::*;

    let naive_local_now = chrono::Local::now().naive_local();

    let find_token = user_token_ref
        .filter(token.eq(target_token))
        .filter(expiration_at.gt(naive_local_now))
        .execute(conn).unwrap();

    match find_token as i32 {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ServiceError::BadRequest("Duplicate token found.".to_string())),
    }
}

/// get the uuid_user who owns the token
pub(crate) fn whose_token(
    target_token: &str,
    conn: &PooledConnection,
) -> Result<Uuid, ServiceError> {
    use crate::schema::user_token_ref::dsl::*;

    user_token_ref
        .filter(token.eq(target_token))
        .select(uuid_user)
        .first(conn)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
