use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    model::{IptComponentData, InsertableComponent},
    access::util::check_access_component_for_user,
};
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component(
    logged_user_uuid: &Uuid,
    data: &IptComponentData,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    if let Some(ref parent_component_uuid) = data.parent_component_uuid {
        check_access_component_for_user(
            logged_user_uuid,
            parent_component_uuid,
            &3, // need_access_level
            conn
        )?;
    }

    let mut insert_data: InsertableComponent = data.into();
    // set main image component
    insert_data.set_image_uuid();
    // set logged user as owner component
    insert_data.set_user_uuid(logged_user_uuid);

    match insert_data.parent_uuid_is_nil() {
        true => {
            insert_data.parent_uuid_to_base();

            let new_uuid = diesel::insert_into(component_ref::component_ref)
                .values(&insert_data)
                .returning(component_ref::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error create component data: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            diesel::update(component_ref::component_ref)
                .filter(component_ref::uuid.eq(&new_uuid))
                .set(component_ref::parent_component_uuid.eq(&new_uuid))
                .returning(component_ref::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error change parent component uuid: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        false => diesel::insert_into(component_ref::component_ref)
            .values(&insert_data)
            .returning(component_ref::uuid)
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed created standard: {:?}", err);
                ServiceError::InternalServerError
        }),
    }
}
