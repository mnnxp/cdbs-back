use crate::errors::{ServiceResult, ServiceError};
use crate::database::{get_conn, PooledConnection};
use crate::models::user::model::User;
use crate::models::relate_ref::language::model::SetLang;

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
pub(crate) fn check_authorized(cxt: &Context<'_>) -> ServiceResult<bool> {
    use crate::models::user::access::token::{token_from_cxt, check_token};

    let conn: &PooledConnection = &get_conn(cxt)?;

    let token = token_from_cxt(cxt)?;

    match check_token(token.as_str(), conn)? {
        true => Ok(true),
        false => Err(ServiceError::Unauthorized),
    }
}

/// get user_uuid of the authorized user
/// with and without checking valid token
pub(crate) fn get_logged_user_uuid(
    cxt: &Context<'_>,
    need_check: bool
) -> ServiceResult<Uuid> {
    use crate::models::user::access::token::{token_from_cxt, whose_token, check_token};

    let target_token = token_from_cxt(cxt)?;

    let conn: &PooledConnection = &get_conn(cxt)?;

    match need_check {
        false => whose_token(target_token.as_str(), conn),
        true => {
            if check_token(target_token.as_str(), conn)? {
                whose_token(target_token.as_str(), conn)
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
// pub(crate) fn compare_user_uuid(target_auth_user_uuid: Uuid, cxt: &Context<'_>) -> ServiceResult<bool> {
//     Ok(get_auth_user_uuid(cxt, false)? == target_auth_user_uuid)
// }
