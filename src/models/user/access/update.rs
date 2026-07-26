use crate::errors::{ServiceError, ServiceResult};
use diesel::prelude::*;
use uuid::Uuid;

/// Sets the user profile access level for other users
pub(crate) fn change_access_type_user(
    logged_user_uuid: &Uuid,
    new_type_access: i32,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::user_ref::dsl as user_ref;

    let get_access = diesel::update(user_ref::user_ref)
        .filter(
            user_ref::uuid
                .eq(logged_user_uuid)
                .and(user_ref::type_access_id.ne(new_type_access)),
        )
        .set(user_ref::type_access_id.eq(new_type_access))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed change access for user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(get_access == 1)
}
