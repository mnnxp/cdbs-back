use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{SlimUser, User};
use crate::models::user::access::{
    model::UserToken, hash::verify,
};
use diesel::prelude::*;

// Login user with user/pass, return new token
pub(crate) fn login_with_pass(
    username: &str,
    password: &str,
    conn: &PgConnection,
) -> ServiceResult<UserToken> {
    use crate::models::user::access::token::{generate, decode, write_token};

    let slim_user = login_check(
        username,
        password,
        conn
    )?;

    // serde_json::to_string(&slim_user)
    //     .map_err(|_| ServiceError::InternalServerError)?;

    // generate new token for user
    let new_token = generate(&slim_user)?;

    match new_token.bearer {
        None => Err(ServiceError::InternalServerError),
        Some(ref token) => {
            // decrypt new token
            let new_data = decode(token.as_str())?;

            // insert data new token into the table
            write_token(&slim_user.uuid, token, new_data, conn)
        }
    }
}

/// Compare password with password in database
/// if check success return SlimUser for generate token
fn login_check(
    username: &str,
    password: &str,
    conn: &PgConnection,
) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl as user_ref;

    let user = user_ref::user_ref
        .filter(user_ref::username.eq(username))
        .select((
            user_ref::uuid,
            user_ref::psw_hash,
            user_ref::psw_salt,
            user_ref::username,
            user_ref::program_id,
        ))
        .first::<User>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    match verify(user.get_psw_hash(), user.get_psw_salt(), password.as_bytes()) {
        true => Ok(user.into()),
        false => Err(ServiceError::Unauthorized),
    }
}
