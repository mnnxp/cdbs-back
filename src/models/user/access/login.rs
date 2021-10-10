use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{SlimUser, User};
use crate::models::user::access::{
    model::UserToken, util::verify,
};
use diesel::prelude::*;

// Login user with user/pass, return new token
pub(crate) fn login_with_pass(
    user_username: &str,
    user_password: &str,
    conn: &PgConnection,
) -> ServiceResult<UserToken> {
    use crate::models::user::access::token::{generate, decode, write_token};

    let slim_user = login_check(
        user_username,
        user_password,
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
            write_token(token, new_data, conn)
        }
    }
}

fn login_check(
    user_username: &str,
    user_password: &str,
    conn: &PgConnection,
) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::{username, user_ref};

    let user = user_ref
        .filter(username.eq(user_username))
        .first::<User>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    match verify(user.get_psw_hash(), user.get_psw_salt(), user_password.as_bytes()) {
        true => Ok(user.into()),
        false => Err(ServiceError::Unauthorized),
    }
}
