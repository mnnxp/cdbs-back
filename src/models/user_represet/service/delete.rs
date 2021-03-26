use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult};
// use crate::models::user::model::{LoggedUser, SlimUser};
use crate::models::user_represet::model::{
    // InsertableUserRepreset,
    SlimUserRepreset,
    UserRepreset
    // UserRepresetData
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete(
    user_uuid: Uuid,
    uuid_represet_delete: Uuid,
    pool: web::Data<Pool>
) -> ServiceResult<SlimUserRepreset> {
    let conn = &db_connection(&pool)?;

    delete_user_represet(
        user_uuid,
        uuid_represet_delete,
        conn
    )
}

fn delete_user_represet(
    user_uuid: Uuid,
    uuid_represet_delete: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimUserRepreset> {
    use crate::schema::user_represet_ref::dsl::*;

    // debug!("fn user_uuid = {}", &user_uuid);
    // debug!("fn uuid_represet_delete = {}", &uuid_represet_delete);

    let delete_user_represet: UserRepreset = user_represet_ref
        .filter(uuid_user.eq(user_uuid))
        .filter(uuid.eq(uuid_represet_delete))
        .get_result(conn)?;

    diesel::delete(user_represet_ref.filter(uuid.eq(delete_user_represet.uuid))).execute(conn)?;

    // debug!("fn delete_user_represet ={:?}", &delete_user_represet);

    Ok(delete_user_represet.into())
}
