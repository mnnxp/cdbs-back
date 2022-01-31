use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{IptUserData, InsertableUser, SlimUser};
use crate::models::user::util::check_use_username;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
// use uuid::Uuid;

/// Create new user
pub(crate) fn create_user(
    data: &IptUserData,
    conn: &PgConnection
) -> ServiceResult<SlimUser> {
    if check_use_username(&data.username, conn)? {
        return Err(ServiceError::BadRequest(
            "This username is already used".to_string()
        ));
    }

    let insert_values: InsertableUser = data.into();

    diesel::insert_into(user_ref::user_ref)
        .values(&insert_values)
        .returning((
            user_ref::uuid,
            user_ref::username,
            user_ref::program_id,
        ))
        .get_result::<SlimUser>(conn)
        .map_err(|err| {
            debug!("Failed create new user: {:?}", err);
            ServiceError::InternalServerError
        })
}
