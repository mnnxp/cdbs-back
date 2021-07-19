// use crate::database::{db_connection, Pool};
// use crate::errors::{
//     // ServiceError,
//     ServiceResult
// };
// use crate::models::component_modification::model::{
//     SlimComponentModification,
//     ComponentModification
// };
// // use crate::models::user::util::verify;
// use actix_web::web;
// use diesel::prelude::*;
// use uuid::Uuid;
//
// pub fn delete(
//     user_uuid: Uuid,
//     component_modification_delete: Uuid,
//     pool: web::Data<Pool>
// ) -> ServiceResult<SlimComponentModification> {
//     let conn = &db_connection(&pool)?;
//
//     delete_component_modification(user_uuid, component_modification_delete, conn)
// }
//
// fn delete_component_modification(
//     user_uuid: Uuid,
//     component_modification_delete: Uuid,
//     conn: &PgConnection
// ) -> ServiceResult<SlimComponentModification> {
//     use crate::schema::component_modification_list::dsl::*;
//
//     // debug!("fn user_uuid = {}", &user_uuid);
//     // debug!("fn component_modification_delete = {}", &component_modification_delete);
//
//     let delete_component_modification: ComponentModification = component_modification_list
//         .filter(uuid.eq(user_uuid))
//         .filter(uuid.eq(component_modification_delete))
//         .get_result(conn)?;
//
//     diesel::delete(
//         component_modification_list.filter(uuid.eq(delete_component_modification.uuid))
//     ).execute(conn)?;
//
//     // debug!("fn delete_component_modification ={:?}", &delete_component_modification);
//
//     Ok(delete_component_modification.into())
// }
