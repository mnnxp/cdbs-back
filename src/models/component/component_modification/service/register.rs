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

    let data: InsertableComponentModification = data.into();

    diesel::insert_into(component_modification_list::component_modification_list)
        .values(&data)
        .returning(component_modification_list::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Error create modification data: {:?}", err);
            ServiceError::InternalServerError
        })
}
