use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
// use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use crate::models::component::model::{InsertableComponent, SlimComponent, Component, ComponentData};
use actix_web::web;
use diesel::prelude::*;

pub(crate) fn register(component_data: ComponentData, pool: web::Data<Pool>) -> ServiceResult<SlimComponent> {
    let conn = &db_connection(&pool)?;
    create_component(component_data, conn)
}

pub(crate) fn create_component(component_data: ComponentData, conn: &PgConnection) -> ServiceResult<SlimComponent> {
    use crate::schema::component_ref::dsl::component_ref;

    let component: InsertableComponent = component_data.into();
    let inserted_component: Component = diesel::insert_into(component_ref).values(&component).get_result(conn)?;
    Ok(inserted_component.into())
}
