use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::model::IptUpdateComponentModificationData;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет основные данные модификации компонента.
pub(crate) fn update_modification_data(
    logged_user_uuid: &Uuid,
    target_modification_uuid: &Uuid,
    data: &IptUpdateComponentModificationData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(target_modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column modification_name
    if let Some(value) = &data.modification_name {
        count_update_columns += diesel::update(component_modification_list::component_modification_list
            .filter(component_modification_list::uuid.eq(target_modification_uuid)
            .and(component_modification_list::modification_name.ne(value))))
            .set(component_modification_list::modification_name.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(component_modification_list::component_modification_list
            .filter(component_modification_list::uuid.eq(target_modification_uuid)
            .and(component_modification_list::description.ne(value))))
            .set(component_modification_list::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column actual_status_id
    if let Some(value) = &data.actual_status_id {
        count_update_columns += diesel::update(component_modification_list::component_modification_list
            .filter(component_modification_list::uuid.eq(target_modification_uuid)
            .and(component_modification_list::actual_status_id.ne(value))))
            .set(component_modification_list::actual_status_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(ServiceError::BadRequest("The data has already".to_string()));
    }

    diesel::update(component_modification_list::component_modification_list
        .filter(component_modification_list::uuid.eq(target_modification_uuid)))
        .set(component_modification_list::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            ServiceError::BadRequest("Failed update data".to_string())
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
