use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::models::user_represent::model::{InsertableUserRepresent, SlimUserRepresent, UserRepresent, UserRepresentData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(user_represent_data: UserRepresentData, pool: web::Data<Pool>) -> ServiceResult<SlimUserRepresent> {
    let conn = &db_connection(&pool)?;
    create_user_represent(user_represent_data, conn)
}

pub fn create_user_represent(
    user_represent_data: UserRepresentData,
    conn: &PgConnection
) -> ServiceResult<SlimUserRepresent> {
    use crate::schema::user_represent_ref::dsl::user_represent_ref;

    let user_represent: InsertableUserRepresent = user_represent_data.into();
    let inserted_user_represent: UserRepresent = diesel::insert_into(user_represent_ref).values(&user_represent).get_result(conn)?;
    Ok(inserted_user_represent.into())
}
