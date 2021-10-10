use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::access::password::check_password;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete all data user
pub(crate) fn delete_user(
    logged_user_uuid: &Uuid,
    user_password: &[u8],
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::user_ref::dsl as user_ref;

    // Access check:
    // compare password with password in database
    check_password(
        logged_user_uuid,
        user_password,
        conn
    )?;

    let res_delete = diesel::delete(user_ref::user_ref)
        .filter(user_ref::uuid.eq(logged_user_uuid))
        .execute(conn);

    match res_delete {
        Ok(x) => {
            debug!("User del: {:?}", x);

            Ok(true)
        },
        Err(err) => {
            debug!("Failed delete user data: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed delete user data".to_string()
            ))
        },
    }
}
