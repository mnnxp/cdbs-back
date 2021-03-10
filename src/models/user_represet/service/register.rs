use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::models::user_represet::model::{InsertableUserRepreset, SlimUserRepreset, UserRepreset, UserRepresetData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(user_represet_data: UserRepresetData, pool: web::Data<Pool>) -> ServiceResult<SlimUserRepreset> {
    let conn = &db_connection(&pool)?;
    create_user_represet(user_represet_data, conn)
}

pub fn create_user_represet(user_represet_data: UserRepresetData, conn: &PgConnection) -> ServiceResult<SlimUserRepreset> {
    use crate::schema::user_represet_ref::dsl::user_represet_ref;

    let user_represet: InsertableUserRepreset = user_represet_data.into();
    let inserted_user_represet: UserRepreset = diesel::insert_into(user_represet_ref).values(&user_represet).get_result(conn)?;
    Ok(inserted_user_represet.into())
}
