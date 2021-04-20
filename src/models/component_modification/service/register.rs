use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component_modification::model::{
    InsertableComponentModification,
    SlimComponentModification,
    ComponentModification,
    ComponentModificationData
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn register(
    new_modification_data: ComponentModificationData,
    user_uuid: Uuid,
    // component_parent_uuid: Uuid,
    pool: web::Data<Pool>
) -> ServiceResult<SlimComponentModification> {
    let conn = &db_connection(&pool)?;
    create_component_modification(new_modification_data, user_uuid, conn)
}

pub(crate) fn create_component_modification(
    new_modification_data: ComponentModificationData,
    user_uuid: Uuid,
    // component_parent_uuid: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    use crate::schema::component_ref::dsl::*;
    use crate::schema::component_ref::dsl::uuid as uuid_component;
    use crate::schema::component_modification_list::dsl::*;
    use diesel::dsl::count;

    let flag_found_component: i64 = component_ref
        .filter(uuid_user.eq(user_uuid))
        .filter(uuid_component.eq(new_modification_data.uuid_component))
        .select(count(uuid_component))
        .first(conn).unwrap();

    // debug!("fn create_component_modification START SEARCH ={:?}", flag_found_component);

    match flag_found_component {
        0 => Err(ServiceError::BadRequest("Not found this component of you.".to_string())),
        1 => {
            let new_modification_data: InsertableComponentModification = new_modification_data.into();
            let inserted_modification_data: ComponentModification = diesel::insert_into(
                component_modification_list)
                .values(&new_modification_data)
                .get_result(conn)?;
            Ok(inserted_modification_data.into())
        }
        _ => Err(ServiceError::BadRequest("Wow, what's?".to_string())),
    }
}
