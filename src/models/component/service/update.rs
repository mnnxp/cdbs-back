use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::model::IptUpdateComponentData;
use crate::models::component::util::{
    check_is_owner, check_access_component_for_user
};
use diesel::prelude::*;
use uuid::Uuid;

/// Update component main data by uuid
pub(crate) fn update_component_by_uuid(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    data: &IptUpdateComponentData,
    conn: &PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::component_ref::dsl as component_ref;

    // need top level access for change component main data
    let need_access_level = 1;

    let target_component_uuid: &Uuid = match check_is_owner(
        logged_user_uuid,
        target_component_uuid,
        conn
    ) {
        // component owner user
        true => target_component_uuid,
        // need check access if user not owned component
        false => {
            debug!("User not owned target component");

            if !check_access_component_for_user(
                logged_user_uuid,
                target_component_uuid,
                &need_access_level,
                false, // <-- not need check owned again
                conn,
            )? {
                // return error if user not have access level
                return Err(ServiceError::BadRequest("Access denied".to_string()))
            }

            // initialization uuid if found target access level
            target_component_uuid
        }
    };


    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column parent_component_uuid
    if let Some(value) = &data.parent_component_uuid {
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::parent_component_uuid.ne(&value))))
            .set(component_ref::parent_component_uuid.eq(value))
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

    // update column name
    if let Some(value) = &data.name {
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::name.ne(&value))))
            .set(component_ref::name.eq(value))
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
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::description.ne(&value))))
            .set(component_ref::description.eq(value))
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

    // update column type_access_id
    if let Some(value) = &data.type_access_id {
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::type_access_id.ne(&value))))
            .set(component_ref::type_access_id.eq(value))
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

    // update column component_type_id
    if let Some(value) = &data.component_type_id {
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::component_type_id.ne(&value))))
            .set(component_ref::component_type_id.eq(value))
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
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::actual_status_id.ne(&value))))
            .set(component_ref::actual_status_id.eq(value))
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

    // new date for updated_at in component_ref table if update more one column
    if count_update_columns > 0 {
        let res = diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)))
            .set(component_ref::updated_at.eq(chrono::Local::now().naive_local()))
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
