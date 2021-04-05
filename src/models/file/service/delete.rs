// use crate::database::{db_connection, Pool};
// use crate::errors::{
//     // ServiceError,
//     ServiceResult
// };
// use crate::models::file::model::{SlimFile, File};
// // use crate::models::user::util::verify;
// use actix_web::web;
// use diesel::prelude::*;
// use uuid::Uuid;
//
// pub fn delete(
//     user_uuid: Uuid,
//     file_delete: Uuid,
//     pool: web::Data<Pool>
// ) -> ServiceResult<SlimFile> {
//     let conn = &db_connection(&pool)?;
//
//     delete_file(
//         user_uuid,
//         file_delete,
//         conn
//     )
// }
//
// fn delete_file(
//     user_uuid: Uuid,
//     file_delete: Uuid,
//     conn: &PgConnection
// ) -> ServiceResult<SlimFile> {
//     use crate::schema::file_ref::dsl::*;
//
//     // debug!("fn user_uuid = {}", &user_uuid);
//     // debug!("fn file_delete = {}", &file_delete);
//
//     let delete_file: File = file_ref
//         .filter(uuid_user_create.eq(user_uuid))
//         .filter(uuid.eq(file_delete))
//         .get_result(conn)?;
//
//     diesel::delete(file_ref.filter(uuid.eq(delete_file.uuid))).execute(conn)?;
//
//     // debug!("fn delete_file ={:?}", &delete_file);
//
//     Ok(delete_file.into())
// }
//
//
// fn delete_files_vec(
//     user_uuid: Uuid,
//     file_delete: Vec<Uuid>,
//     conn: &PgConnection
// ) -> ServiceResult<Vec<Uuid>> {
//     use crate::schema::file_ref::dsl::*;
//
//     // debug!("fn user_uuid = {}", &user_uuid);
//     // debug!("fn file_delete = {}", &file_delete);
//
//     let uuid_for_delete_file: Vec<Uuid> = file_ref
//         .filter(uuid_user_create.eq(user_uuid))
//         .filter(uuid.eq_any(file_delete))
//         .select(uuid)
//         .load(conn)?;
//
//     diesel::delete(file_ref.filter(uuid.eq_any(uuid_for_delete_file))).execute(conn)?;
//
//     // debug!("fn delete_file ={:?}", &delete_file);
//
//     Ok(delete_file)
// }
