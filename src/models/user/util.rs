use super::model::{
    LoggedUser,
    // SlimUser,
    User,
};
use crate::errors::ServiceError;
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

pub(crate) fn hash_authorized(user: &LoggedUser) -> Result<bool, ServiceError> {
    match user.0 {
        None => Err(ServiceError::Unauthorized),
        Some(_) => Ok(true),
    }
}

pub(crate) fn get_uuid_user(user: &LoggedUser) -> Result<Uuid, ServiceError> {
    match user.0 {
        None => Err(ServiceError::Unauthorized),
        Some(ref user) => Ok(user.uuid),
        // _ => Err(ServiceError::BadRequest("Uuid not correct.".to_string())),
        // Some(ref user) => Err(ServiceError::BadRequest(format!("Uuid not correct. UUID1: {}, UUID2: {};", user.uuid, uuid_user))),
    }
}

// pub fn verify_uuid_user(user: &LoggedUser, uuid_user: Uuid) -> Result<bool, ServiceError> {
//     match user.0 {
//         None => Err(ServiceError::Unauthorized),
//         Some(ref user) if user.uuid == uuid_user => Ok(true),
//         _ => Err(ServiceError::BadRequest("Uuid not correct.".to_string())),
//         // Some(ref user) => Err(ServiceError::BadRequest(format!("Uuid not correct. UUID1: {}, UUID2: {};", user.uuid, uuid_user))),
//     }
// }
