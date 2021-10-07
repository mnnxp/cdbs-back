use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{SlimUser, User};
use crate::models::user::access::util::verify;
// use actix_web::web;
use diesel::prelude::*;

pub(crate) fn login(
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
