use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user_represet::model::{SlimUserRepreset, UserRepreset};
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

    // find represet and check privileges for delete
    let find_represet: i32 = user_represet_ref
        .filter(uuid_user.eq(user_uuid))
        .filter(uuid.eq(uuid_represet_delete))
        .select(id)
        .first(conn).unwrap_or(0);

    match find_represet {
        1..=i32::MAX => {
            // delete represet and save delete data for send response
            let delete_user_represet: UserRepreset =
                diesel::delete(user_represet_ref.filter(id.eq(find_represet))).get_result(conn)?;
            // debug!("fn delete_user_represet ={:?}", &delete_user_represet);
            Ok(delete_user_represet.into())
        },
        _ => Err(ServiceError::BadRequest("The representative not you or not found.".to_string())),
    }
}
