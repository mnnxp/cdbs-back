use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::database::{get_conn, PooledConnection};
use crate::models::user::{
    model::SlimUser,
    access::model::{UserToken, InsertableUserToken},
};
use crate::jwt::model::{Token, Claims};
use crate::schema::user_token_ref::dsl as user_token_ref;
use async_graphql::Context;
// use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// get token from request
pub(crate) fn token_from_cxt(
    cxt: &Context<'_>
) -> ServiceResult<String> {
    let token = match cxt.data_opt::<Token>() {
        Some(token) => token.clone(),
        None => Token { bearer: None },
    };
    match token.bearer {
        Some(bearer) => Ok(bearer),
        None => Err(get_err_msg(ErrorMessage::TokenNotFound)),
    }
}

/// show all tokens for user_uuid
pub(crate) fn show_tokens(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserToken>> {
    user_token_ref::user_token_ref
        .filter(user_token_ref::user_uuid.eq(logged_user_uuid))
        .load(conn)
        .map_err(|err| {
            debug!("Failed get token {:?}", err);
            ServiceError::InternalServerError
        })
}

/// get SlimUser from Claims
pub(crate) fn get_slim_user(
    jwt: Claims
) -> ServiceResult<SlimUser> {
    SlimUser::try_from(jwt)
        .map_err(|_| get_err_msg(ErrorMessage::FailGetUserData))
}

/// updating a token with or without removing the old one
pub(crate) fn update(
    cxt: &Context<'_>,
    flag_delete_token: bool,
) -> ServiceResult<Token> {
    use crate::models::user::access::token::{generate, decode};

    let conn: &mut PooledConnection = &mut get_conn(cxt)?;

    // get old token
    let old_token = token_from_cxt(cxt)?;
    if !check_token(old_token.as_str(), conn)? {
        return Err(get_err_msg(ErrorMessage::TokenIsInvalid))
    }

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
            write_token(&user.uuid, token, new_data, conn)?;
            Ok(new_token)
        }
    }
}

/// delete token to table user_token_ref of database
pub(crate) fn delete_token(
    target_token: &str,
    conn: &mut PgConnection
) -> ServiceResult<UserToken> {
    diesel::delete(user_token_ref::user_token_ref)
        .filter(user_token_ref::token.eq(&target_token))
        .get_result::<UserToken>(conn)
        .map_err(|err| {
            debug!("Failed delete token: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// delete target token to table user_token_ref of database
pub(crate) fn delete_user_token(
    logged_user_uuid: &Uuid,
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let delete_token = diesel::delete(user_token_ref::user_token_ref)
        .filter(user_token_ref::user_uuid.eq(&logged_user_uuid)
        .and(user_token_ref::token.eq(&target_token)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete token: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(delete_token > 0)
}

/// delete tokens to table user_token_ref of database
pub(crate) fn delete_all_tokens(
    target_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let del_tokens = diesel::delete(user_token_ref::user_token_ref)
        .filter(user_token_ref::user_uuid.eq_all(target_user_uuid))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete tokens: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(del_tokens)
}

/// write token to table user_token_ref of database
pub(crate) fn write_token(
    logged_user_uuid: &Uuid,
    new_token: &str,
    jwt: Claims,
    conn: &mut PgConnection,
) -> ServiceResult<UserToken> {
    // get duplicate token
    let check_token = user_token_ref::user_token_ref
        .filter(user_token_ref::token.eq(new_token))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check token: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // check for no duplicate token
    match check_token {
        // creating a structure for writing token to a table
        0 => {
            let mut data = InsertableUserToken::new(logged_user_uuid, &jwt);
            data.put_token(new_token);
            diesel::insert_into(user_token_ref::user_token_ref)
                .values(&data)
                .get_result::<UserToken>(conn)
                .map_err(|err| {
                    debug!("Failed check token: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        1 => Err(get_err_msg(ErrorMessage::PleaseTryAgainLater)),
        _ => Err(ServiceError::InternalServerError), // found duplicates token
    }
}

/// check token for validity
pub(crate) fn check_token(
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let naive_local_now = chrono::Local::now().naive_local();

    let get_token = user_token_ref::user_token_ref
        .filter(user_token_ref::token.eq(target_token)
        .and(user_token_ref::expiration_at.gt(naive_local_now)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check token: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_token {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(get_err_msg(ErrorMessage::FoundDuplicateToken)),
    }
}

/// get the user_uuid who owns the token
pub(crate) fn whose_token(
    target_token: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    user_token_ref::user_token_ref
        .filter(user_token_ref::token.eq(target_token))
        .select(user_token_ref::user_uuid)
        .first(conn)
        .map_err(|err| {
            debug!("Failed get token: {:?}", err);
            ServiceError::InternalServerError
        })
}
