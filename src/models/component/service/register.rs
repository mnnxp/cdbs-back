use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::component_model::IptComponentData;
use crate::models::component::{
    access::util::check_access_component_for_user,
    component_modification::{
        model::InsertableComponentModification, service::register::single_modification,
    },
    model::InsertableComponent,
};
use crate::models::user::component_fav::service::add::component_to_fav_ft;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Создает компонент, возвращает UUID нового компонента.
pub(crate) fn create_component(
    logged_user_uuid: &Uuid,
    data: &IptComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    if let Some(ref parent_component_uuid) = data.parent_component_uuid {
        check_access_component_for_user(
            logged_user_uuid,
            parent_component_uuid,
            &3, // need_access_level
            conn,
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

            // add the new component to the user's favorites
            component_to_fav_ft(logged_user_uuid, &new_uuid, conn)?;
            // add a new modification to the component
            insert_new_modifiacation(&new_uuid, conn)?;

            diesel::update(component_ref::component_ref)
                .filter(component_ref::uuid.eq(&new_uuid))
                .set(component_ref::parent_component_uuid.eq(&new_uuid))
                .returning(component_ref::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error change parent component uuid: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        false => diesel::insert_into(component_ref::component_ref)
            .values(&insert_data)
            .returning(component_ref::uuid)
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed created component: {:?}", err);
                ServiceError::InternalServerError
            }),
    }
}

fn insert_new_modifiacation(component_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<Uuid> {
    let mut insert_data =
        InsertableComponentModification::get_default_for_component(component_uuid);
    single_modification(&mut insert_data, conn)
}
