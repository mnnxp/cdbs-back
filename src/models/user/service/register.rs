use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{IptUserData, InsertableUser, SlimUser};
use diesel::prelude::*;
// use uuid::Uuid;

/// Create new user
pub(crate) fn create_user(
    data: &IptUserData,
    conn: &PgConnection
) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::*;

    let insert_values: InsertableUser = data.into();

    let inserted_user = diesel::insert_into(user_ref)
        .values(&insert_values)
        .returning((
            uuid,
            program_id,
            username,
        ))
        .get_result::<SlimUser>(conn);

    match inserted_user {
        Ok(x) => {
            debug!("Completed create new user: {:?}", x);
            Ok(x)
        },
        Err(err) => {
            debug!("Failed create new user: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed create new user".to_string()
            ))
        },
    }
}
