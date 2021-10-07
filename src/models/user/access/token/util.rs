use crate::errors::{ServiceResult, ServiceError};
use super::model::{UserToken, InsertableUserToken};
use crate::database::{get_conn, PooledConnection};
use crate::models::user::model::SlimUser;
use crate::jwt::model::{Token, Claims};
use std::convert::TryFrom;
use async_graphql::Context;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// get token from request
pub(crate) fn token_from_cxt(cxt: &Context<'_>) -> ServiceResult<String> {
    let token = match cxt.data_opt::<Token>() {
        Some(token) => token.clone(),
        None => Token { bearer: None },
    };
    match token.bearer {
        Some(bearer) => Ok(bearer),
        None => Err(ServiceError::BadRequest("Token not found.".to_string())),
    }
}

/// show all tokens for user_uuid
pub(crate) fn show_tokens(
    cxt: &Context<'_>,
    auth_user_uuid: Uuid,
) -> ServiceResult<Vec<UserToken>> {
    let conn: &PooledConnection = &get_conn(cxt)?;

    use crate::schema::user_token_ref::dsl::*;

    user_token_ref
        .filter(user_uuid.eq(auth_user_uuid))
        .load(conn)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

/// get SlimUser from Claims
pub(crate) fn get_slim_user(jwt: Claims) -> ServiceResult<SlimUser> {
    SlimUser::try_from(jwt)
        .map_err(|_| ServiceError::BadRequest("Fail get SlimUser from Claims.".to_string()))
}

/// updating a token with or without removing the old one
pub(crate) fn update(
    cxt: &Context<'_>,
    flag_delete_token: bool
) -> ServiceResult<Token> {
    use crate::models::user::access::token::{generate, decode};

    let conn: &PooledConnection = &get_conn(cxt)?;

    // get old token
    let old_token = token_from_cxt(cxt)?;
    if check_token(old_token.as_str(), conn)? {
        // decrypt old token
        let old_data = decode(old_token.as_str())?;
        if flag_delete_token {
            // deactivate old token
            delete_token(old_token.as_str(), conn)?;
        }

        // get data from old token
        let user = get_slim_user(old_data)?;

        // creating a new token
        let new_token = generate(&user)?;

        // decrypt new token
        let new_data = decode(old_token.as_str())?;

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
pub(crate) fn delete_token(
    target_token: &str,
    conn: &PgConnection
) -> ServiceResult<UserToken> {
    use crate::schema::user_token_ref::dsl::*;

    let updated_token: UserToken = diesel::delete(user_token_ref)
        .filter(token.eq(&target_token))
        .get_result(conn)?;

    Ok(updated_token)
}

/// delete target token to table user_token_ref of database
pub(crate) fn delete_user_token(
    cxt: &Context<'_>,
    target_token: &str,
    auth_user_uuid: Uuid,
) -> ServiceResult<i32> {
    let conn: &PooledConnection = &get_conn(cxt)?;

    use crate::schema::user_token_ref::dsl::*;

    let updated_token: usize = diesel::delete(user_token_ref)
        .filter(user_uuid.eq(&auth_user_uuid))
        .filter(token.eq(&target_token))
        .execute(conn)?;

    Ok(updated_token as i32)
}

/// delete tokens to table user_token_ref of database
pub(crate) fn delete_all_tokens(
    cxt: &Context<'_>,
    target_user_uuid: Uuid,
) -> ServiceResult<i32> {
    let conn: &PooledConnection = &get_conn(cxt)?;

    use crate::schema::user_token_ref::dsl::*;

    let updated_token: usize = diesel::delete(user_token_ref)
        .filter(user_uuid.eq_all(&target_user_uuid))
        .execute(conn)?;

    Ok(updated_token as i32)
}

/// write token to table user_token_ref of database
pub(crate) fn write_token(
    new_token: &str,
    jwt: Claims,
    conn: &PgConnection,
) -> ServiceResult<UserToken> {
    use crate::schema::user_token_ref::dsl::user_token_ref;
    use crate::schema::user_token_ref::dsl::token;

    // find duplicate token
    let find_token = user_token_ref
        .filter(token.eq(new_token))
        .execute(conn);

    // check for no duplicate token
    match find_token {
        Ok(0) => {
            // creating a structure for writing token to a table
            let user_token = InsertableUserToken {
                user_uuid: Uuid::parse_str(&jwt.sub)?,
                token: new_token.to_string(),
                created_at: NaiveDateTime::from_timestamp(jwt.iat, 0),
                expiration_at: NaiveDateTime::from_timestamp(jwt.exp, 0),
            };

            let inserted_token: UserToken = diesel::insert_into(user_token_ref)
            .values(&user_token)
            .get_result(conn)?;
            Ok(inserted_token)
        },
        Ok(1) => Err(ServiceError::BadRequest("Please, try again later.".to_string())),
        Ok(_) => Err(ServiceError::BadRequest("Duplicate token found.".to_string())),
        Err(err) => {
            debug!("Failed check token: {:?}", err);

            Err(ServiceError::InternalServerError)
        },
    }
}

/// check token for validity
pub(crate) fn check_token(
    target_token: &str,
    conn: &PgConnection,
) -> ServiceResult<bool> {
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

/// get the user_uuid who owns the token
pub(crate) fn whose_token(
    target_token: &str,
    conn: &PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::user_token_ref::dsl::*;

    user_token_ref
        .filter(token.eq(target_token))
        .select(user_uuid)
        .first(conn)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
