// use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::model::{
    InsertableComponentModification,
    IptComponentModificationData,
    SlimComponentModification,
    ComponentModification,
};
// use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component_modification(
    new_modification_data: IptComponentModificationData,
    logged_user_uuid: Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    use crate::schema::component_ref::dsl::*;
    use crate::schema::component_ref::dsl::uuid as component_uuid;
    use crate::schema::component_modification_list::dsl::*;
    use diesel::dsl::count;

    let new_modification_data: InsertableComponentModification = new_modification_data.into();

    let flag_found_component: i64 = component_ref
        .filter(user_uuid.eq(logged_user_uuid))
        .filter(component_uuid.eq(new_modification_data.component_uuid))
        .select(count(component_uuid))
        .first(conn).unwrap();

    // debug!("fn create_component_modification START SEARCH ={:?}", flag_found_component);

    match flag_found_component {
        0 => Err(ServiceError::BadRequest("Access denied".to_string())),
        1 => {
            let inserted_modification_data: ComponentModification = diesel::insert_into(
                component_modification_list)
                .values(&new_modification_data)
                .get_result(conn)?;
            Ok(inserted_modification_data.into())
        }
        _ => Err(ServiceError::BadRequest("Wow what? Found several components.".to_string())),
    }
}
