use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
// use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use crate::file::model::{InsertableFile, SlimFile, File, FileData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(file_data: FileData, pool: web::Data<Pool>) -> ServiceResult<SlimFile> {
    let conn = &db_connection(&pool)?;
    create_file(file_data, conn)
}

pub fn create_file(file_data: FileData, conn: &PgConnection) -> ServiceResult<SlimFile> {
    use crate::schema::file_ref::dsl::file_ref;

    let file: InsertableFile = file_data.into();
    let inserted_file: File = diesel::insert_into(file_ref).values(&file).get_result(conn)?;
    Ok(inserted_file.into())
}
