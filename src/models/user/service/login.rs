use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{SlimUser, User};
use crate::models::user::util::verify;
use actix_web::web;
use diesel::prelude::*;

pub fn login(
    user_username: &str,
    user_password: &str,
    pool: web::Data<Pool>,
) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::{username, user_ref};

    let conn = &db_connection(&pool)?;
    let user = user_ref
        .filter(username.eq(user_username))
        .first::<User>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    if verify(&user, &user_password) {
        Ok(user.into())
    } else {
        Err(ServiceError::Unauthorized)
    }
}
