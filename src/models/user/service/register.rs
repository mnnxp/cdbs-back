// use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::models::user::model::{InsertableUser, SlimUser, User, IptUserData};
// use actix_web::web;
use diesel::prelude::*;

// pub(crate) fn register(user_data: UserData, pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
//     let conn = &db_connection(&pool)?;
//     create_user(user_data, conn)
// }

pub(crate) fn create_user(user_data: IptUserData, conn: &PgConnection) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::user_ref;

    let user: InsertableUser = user_data.into();
    let inserted_user: User = diesel::insert_into(user_ref).values(&user).get_result(conn)?;
    Ok(inserted_user.into())
}
