use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
// use crate::models::component::model::{InsertableComponent, SlimComponent, Component, ComponentData};
use crate::models::component_modification::model::{
    InsertableComponentModification,
    SlimComponentModification,
    ComponentModification,
    ComponentModificationData
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;


pub fn register(
    new_modification_data: ComponentModificationData,
    _user_uuid: Uuid,
    pool: web::Data<Pool>
) -> ServiceResult<SlimComponentModification> {
    let conn = &db_connection(&pool)?;
    create_component_modification(new_modification_data, _user_uuid, conn)
}

pub fn create_component_modification(
    new_modification_data: ComponentModificationData,
    _user_uuid: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    // use crate::schema::component_ref::dsl::component_ref;
    use crate::schema::component_modification_list::dsl::component_modification_list;

    // let component_from_user: Component = component_ref
    //     .filter(uuid_user.eq(user_uuid))
    //     .filter(uuid_component.eq(&new_modification_data.uuid_component_parent))
    //     .get_result(conn)?;
    //
    // debug!("fn delete_user_represet ={:?}", &component_from_user);

    let new_modification_data: InsertableComponentModification = new_modification_data.into();
    let inserted_modification_data: ComponentModification = diesel::insert_into(
        component_modification_list)
        .values(&new_modification_data)
        .get_result(conn)?;
    Ok(inserted_modification_data.into())
}
