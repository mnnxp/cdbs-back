// use crate::errors::ServiceResult;
use diesel::prelude::*;
use uuid::Uuid;

/// Gets user_uuid by username
pub(crate) fn get_uuid_by_username(
    username: &str,
    conn: &PgConnection,
) -> Uuid {
    use crate::schema::user_ref::dsl as user_ref;

    user_ref::user_ref
        .filter(user_ref::username.eq(username))
        .select(user_ref::uuid)
        .first::<Uuid>(conn)
        .expect("Failed found user by username")
}
