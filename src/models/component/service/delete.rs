// use crate::database::{db_connection, Pool};
// use crate::errors::{
//     // ServiceError,
//     ServiceResult
// };
// use crate::models::component::model::{
//     SlimComponent,
//     Component
// };
// // use crate::models::user::util::verify;
// use actix_web::web;
// use diesel::prelude::*;
// use uuid::Uuid;
//
// pub fn delete(
//     user_uuid: Uuid,
//     component_delete: Uuid,
//     pool: web::Data<Pool>
// ) -> ServiceResult<SlimComponent> {
//     let conn = &db_connection(&pool)?;
//
//     delete_component(user_uuid, component_delete, conn)
// }
//
// fn delete_component(
//     user_uuid: Uuid,
//     component_delete: Uuid,
//     conn: &PgConnection
// ) -> ServiceResult<SlimComponent> {
//     use crate::schema::component_ref::dsl::*;
//
//     // debug!("fn user_uuid = {}", &user_uuid);
//     // debug!("fn component_delete = {}", &component_delete);
//
//     let delete_component: Component = component_ref
//         .filter(uuid.eq(user_uuid))
//         .filter(uuid.eq(component_delete))
//         .get_result(conn)?;
//
//     diesel::delete(
//         component_ref.filter(uuid.eq(delete_component.uuid))
//     ).execute(conn)?;
//
//     // debug!("fn delete_component ={:?}", &delete_component);
//
//     Ok(delete_component.into())
// }
