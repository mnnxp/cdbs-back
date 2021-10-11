use diesel::prelude::*;
use uuid::Uuid;

/// Change access type for user
pub(crate) fn change_access_type_user(
    logged_user_uuid: &Uuid,
    new_type_access: &i32,
    conn: &PgConnection
) -> bool {
    use crate::schema::user_ref::dsl as user_ref;

    let get_access = diesel::update(user_ref::user_ref)
        .filter(user_ref::uuid.eq(logged_user_uuid)
        .and(user_ref::type_access_id.ne(new_type_access)))
        .set(user_ref::type_access_id.eq(new_type_access))
        .execute(conn)
        .expect("Failed change access for user");

    matches!(get_access, 1_usize)
}
