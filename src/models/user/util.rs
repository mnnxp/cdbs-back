use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Get user_uuid by username
pub(crate) fn get_uuid_by_username(
    username: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    user_ref::user_ref
        .filter(user_ref::username.eq(username)
        .and(user_ref::is_enabled.eq(true)
        .and(user_ref::is_delete.eq(false))))
        .select(user_ref::uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get user_uuid by username: {:?}", err);
            get_err_msg(ErrorMessage::DataNotFound)
        })
}

/// Checking if a username already used
pub(crate) fn check_use_username(
    username: &str,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let found = user_ref::user_ref
        .filter(user_ref::username.eq(username))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get by username: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(found == 1)
}
