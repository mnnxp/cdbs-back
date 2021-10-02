// use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::model::{
    InsertableComponentModification,
    IptComponentModificationData,
    SlimComponentModification,
};
// use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_component_modification(
    logged_user_uuid: &Uuid,
    data: &IptComponentModificationData,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    use crate::schema::component_modification_list::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    let data: InsertableComponentModification = data.into();

    let inserted_modification_data = diesel::insert_into(
        component_modification_list)
        .values(&data)
        .returning((
            uuid,
            component_uuid,
            modification_name,
            description,
            updated_at,
        ))
        .get_result::<SlimComponentModification>(conn);

    match inserted_modification_data {
        Ok(idmd) => {
            debug!("Completed inserted data: {:?}", idmd);
            Ok(idmd)
        },
        Err(err) => {
            debug!("Error create modification data: {:?}", err);

            Err(ServiceError::BadRequest(
                "Error create modification data".to_string()
            ))
        },
    }
}
