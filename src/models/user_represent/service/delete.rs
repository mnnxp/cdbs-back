use crate::database::{db_connection, Pool};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user_represent::model::{SlimUserRepresent, UserRepresent};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete(
    user_uuid: Uuid,
    uuid_represent_delete: Uuid,
    pool: web::Data<Pool>
) -> ServiceResult<SlimUserRepresent> {
    let conn = &db_connection(&pool)?;

    delete_user_represent(
        user_uuid,
        uuid_represent_delete,
        conn
    )
}

fn delete_user_represent(
    user_uuid: Uuid,
    uuid_represent_delete: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimUserRepresent> {
    use crate::schema::user_represent_ref::dsl::*;

    // debug!("fn user_uuid = {}", &user_uuid);
    // debug!("fn uuid_represent_delete = {}", &uuid_represent_delete);

    // find represent and check privileges for delete
    let find_represent: i32 = user_represent_ref
        .filter(uuid_user.eq(user_uuid))
        .filter(uuid.eq(uuid_represent_delete))
        .select(id)
        .first(conn).unwrap_or(0);

    match find_represent {
        1..=i32::MAX => {
            // delete represent and save delete data for send response
            let delete_user_represent: UserRepresent =
                diesel::delete(user_represent_ref.filter(id.eq(find_represent))).get_result(conn)?;
            // debug!("fn delete_user_represent ={:?}", &delete_user_represent);
            Ok(delete_user_represent.into())
        },
        _ => Err(ServiceError::BadRequest("The representative not you or not found.".to_string())),
    }
}
