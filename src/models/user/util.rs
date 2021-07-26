use crate::database::{get_conn, PooledConnection};
use super::model::User;
use crate::models::user::service as user;
use crate::errors::ServiceError;
use async_graphql::Context;
use argon2rs::argon2i_simple;
use uuid::Uuid;

pub(crate) fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub(crate) fn make_hash_salt(password: &str, psw_salt: &str) -> [u8; argon2rs::defaults::LENGTH] {
    argon2i_simple(password, psw_salt)
}

pub(crate) fn verify(user: &User, password: &str) -> bool {
    let User {
        psw_hash, psw_salt, ..
    } = user;

    make_hash_salt(password, psw_salt) == psw_hash.as_ref()
}

/// checking user authorization
pub(crate) fn is_authorized(context: &Context<'_>) -> Result<bool, ServiceError> {
    let conn: &PooledConnection = &get_conn(&context)?;
    let token = user::token::token_from_context(&context)?;
    if user::token::check_token(token.as_str(), conn)? {
        Ok(true)
    } else {
        Err(ServiceError::Unauthorized)
    }
}

/// get uuid_user of the authorized user
pub(crate) fn get_auth_uuid_user(context: &Context<'_>) -> Result<Uuid, ServiceError> {
    let conn: &PooledConnection = &get_conn(&context)?;
    let target_token = user::token::token_from_context(&context)?;
    user::token::whose_token(target_token.as_str(), conn)
}

/// comparison of the received uuid_user with the uuid_user of the authorized user
pub(crate) fn compare_uuid_user(target_auth_uuid_user: Uuid, context: &Context<'_>) -> Result<bool, ServiceError> {
    Ok(get_auth_uuid_user(&context)? == target_auth_uuid_user)
}
