use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_modification::model::IptUpdateComponentModificationData;
use crate::models::component::component_modification::util::get_component_by_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Update modification main data by uuid
pub(crate) fn update_modification_data(
    logged_user_uuid: &Uuid,
    target_modification_uuid: &Uuid,
    data: &IptUpdateComponentModificationData,
    conn: &PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::component_modification_list::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(target_modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column modification_name
    if let Some(value) = &data.modification_name {
        let res = diesel::update(component_modification_list
            .filter(uuid.eq(target_modification_uuid)
            .and(modification_name.ne(value))))
            .set(modification_name.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column description
    if let Some(value) = &data.description {
        let res = diesel::update(component_modification_list
            .filter(uuid.eq(target_modification_uuid)
            .and(description.ne(value))))
            .set(description.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column actual_status_id
    if let Some(value) = &data.actual_status_id {
        let res = diesel::update(component_modification_list
            .filter(uuid.eq(target_modification_uuid)
            .and(actual_status_id.ne(value))))
            .set(actual_status_id.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // new date for updated_at in modification_ref table if update more one column
    if count_update_columns > 0 {
        let res = diesel::update(component_modification_list
            .filter(uuid.eq(target_modification_uuid)))
            .set(updated_at.eq(chrono::Local::now().naive_local()))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }

        debug!("Count update columns: {:?}", count_update_columns);

        return Ok(count_update_columns as i32) // <- return count of updates if there are more than 0
    }

    // return error if new data not different with old data
    Err(ServiceError::BadRequest("The data has already".to_string()))
}
