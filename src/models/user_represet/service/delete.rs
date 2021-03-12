use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{LoggedUser, SlimUser};
use crate::models::user_represet::model::{
    InsertableUserRepreset,
    SlimUserRepreset,
    UserRepreset,
    UserRepresetData
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete(
    logged_user: LoggedUser,
    uuid_represets_delete: &str,
    pool: web::Data<Pool>
) -> ServiceResult<SlimUserRepreset> {
    let conn = &db_connection(&pool)?;

    let uuid_represets_delete: Uuid = uuid_represets_delete
        .parse().unwrap_or(Uuid::nil());

    match logged_user.0 {
        None => ServiceResult::Err(ServiceError::Unauthorized),
        Some(user) => delete_user_represet(user.uuid, uuid_represets_delete, conn),
    }
}

fn delete_user_represet(
    user_uuid: Uuid,
    uuid_represets_delete: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimUserRepreset> {
    use crate::schema::user_represet_ref::dsl::*;

    let delete_user_represet: UserRepreset = user_represet_ref
        .filter(uuid_user.eq(user_uuid))
        .filter(uuid.eq(uuid_represets_delete))
        .get_result(conn)?;
    Ok(delete_user_represet.into())
}
