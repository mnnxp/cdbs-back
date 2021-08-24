use crate::errors::ServiceResult;
use crate::models::user::model::SlimUser;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl SlimUser {
    /// get SlimUser data for target uuid user
    pub fn get_by_uuid(
        target_uuid_user: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<SlimUser> {

    Ok(user_ref::user_ref
        .filter(user_ref::uuid.eq(target_uuid_user))
        .select((
            user_ref::uuid,
            user_ref::id_program,
            user_ref::username,
        ))
        .first::<SlimUser>(conn)?)
    }


    /// get SlimUser data for target list uuid user
    pub fn get_list_by_uuids(
        target_list_uuid_user: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimUser>> {

    Ok(user_ref::user_ref
        .filter(user_ref::uuid.eq_any(target_list_uuid_user))
        .select((
            user_ref::uuid,
            user_ref::id_program,
            user_ref::username,
        ))
        .load::<SlimUser>(conn)?)
    }
}
