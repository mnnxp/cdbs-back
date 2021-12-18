use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_modification::model::{
    InsertableComponentModification, IptComponentModificationData
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component_modification(
    logged_user_uuid: &Uuid,
    data: &IptComponentModificationData,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    let mut insert_data: InsertableComponentModification = data.into();

    match insert_data.parent_uuid_is_nil() {
        true => {
            // debug!("insert_data before: {:#?}", insert_data);
            insert_data.parent_uuid_to_base();
            // debug!("insert_data after: {:#?}", insert_data);

            let new_uuid = diesel::insert_into(component_modification_list::component_modification_list)
                .values(&insert_data)
                .returning(component_modification_list::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error create modification data: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            diesel::update(component_modification_list::component_modification_list)
                .filter(component_modification_list::uuid.eq(&new_uuid))
                .set(component_modification_list::parent_modification_uuid.eq(&new_uuid))
                .returning(component_modification_list::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error change parent modification uuid: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        false => {
            diesel::insert_into(component_modification_list::component_modification_list)
                .values(&insert_data)
                .returning(component_modification_list::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error create modification data: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
