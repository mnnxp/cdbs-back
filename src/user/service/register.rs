use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(user_data: UserData, pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
    let conn = &db_connection(&pool)?;
    create_user(user_data, conn)
}

pub fn create_user(user_data: UserData, conn: &PgConnection) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::user_ref;

    let user: InsertableUser = user_data.into();
    let inserted_user: User = diesel::insert_into(user_ref).values(&user).get_result(conn)?;
    Ok(inserted_user.into())
}
