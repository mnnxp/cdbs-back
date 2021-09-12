use crate::database::{get_conn, PooledConnection};
use super::model::User;
use crate::models::user::service as user;
use crate::models::relate_ref::language::model::SetLang;
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
            let idx = rng.gen_range(0..CHARSET.len());
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
pub(crate) fn check_authorized(cxt: &Context<'_>) -> Result<bool, ServiceError> {
    let conn: &PooledConnection = &get_conn(cxt)?;
    let token = user::token::token_from_cxt(cxt)?;

    match user::token::check_token(token.as_str(), conn)? {
        true => Ok(true),
        false => Err(ServiceError::Unauthorized),
    }
}

/// get user_uuid of the authorized user with and without checking
pub(crate) fn get_logged_user_uuid(
    cxt: &Context<'_>,
    need_check: bool
) -> Result<Uuid, ServiceError> {
    let conn: &PooledConnection = &get_conn(cxt)?;
    let target_token = user::token::token_from_cxt(cxt)?;

    match need_check {
        false => user::token::whose_token(target_token.as_str(), conn),
        true => {
            if user::token::check_token(target_token.as_str(), conn)? {
                user::token::whose_token(target_token.as_str(), conn)
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
    }
}

/// get the id of the language for the user interface
/// (if not specified in the request, it will be 1)
pub(crate) fn get_set_language(
    cxt: &Context<'_>,
) -> i32 {
    match cxt.data_opt::<SetLang>() {
        Some(set_lang) => set_lang.lang_id,
        None => 1, // <-- default language
    }
}

// comparison of the received user_uuid with the user_uuid of the authorized user
// pub(crate) fn compare_user_uuid(target_auth_user_uuid: Uuid, cxt: &Context<'_>) -> Result<bool, ServiceError> {
//     Ok(get_auth_user_uuid(cxt, false)? == target_auth_user_uuid)
// }
