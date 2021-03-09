use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::models::component_modification::model::{
    InsertableComponentModification,
    SlimComponentModification,
    ComponentModification,
    ComponentModificationData
};
use actix_web::web;
use diesel::prelude::*;

pub fn register(
    modification_data: ComponentModificationData,
    pool: web::Data<Pool>
) -> ServiceResult<SlimComponentModification> {
    let conn = &db_connection(&pool)?;
    create_component_modification(modification_data, conn)
}

pub fn create_component_modification(
    modification_data: ComponentModificationData,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    use crate::schema::component_modification_list::dsl::component_modification_list;

    let modification_data: InsertableComponentModification = modification_data.into();
    let inserted_modification_data: ComponentModification = diesel::insert_into(
        component_modification_list)
        .values(&modification_data)
        .get_result(conn)?;
    Ok(inserted_modification_data.into())
}
