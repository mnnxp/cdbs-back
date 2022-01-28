use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    param::model::{IptComponentParamsData, InsertableComponentParam},
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::param::model::IptParamData;
use crate::schema::param_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Add new params with values for component
/// or update values an existing component params
pub(crate) fn put_component_params(
    logged_user_uuid: &Uuid,
    data: &IptComponentParamsData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    if data.params.is_empty() {
        return Err(ServiceError::BadRequest(
            "Not found params for adding or updating".to_string()
        ))
    }

    let mut count_changed_rows: usize = 0;

    let mut new_params: Vec<InsertableComponentParam> = Vec::new();      // <-- new parameters to be added
    let mut update_params: Vec<IptParamData> = Vec::new();   // <-- found parameters will be updated

    for param_d in &data.params {
        if param_d.param_id > 0 { // <-- additionally we check the correctness of the id
            let get_param = param_to_component
                .filter(component_uuid.eq(&data.component_uuid)
                .and(param_id.eq(&param_d.param_id)))
                .execute(conn)
                .map_err(|err| {
                    debug!("Fail check param data: {:?} ", err);
                    ServiceError::InternalServerError
                })?;

            match get_param {
                0 => {
                    let insertable_data = InsertableComponentParam {
                        component_uuid: data.component_uuid,
                        param_id: param_d.param_id,
                        value: param_d.value.to_string(),
                    };
                    new_params.push(insertable_data)        // <-- need insert new param
                },
                _ => update_params.push(param_d.clone()),     // <-- already has param need update
            }
        }
    }

    // adding new params
    if !new_params.is_empty() {
        count_changed_rows += adding_new_component_params(&new_params, conn)?;
    }

    // updating params values
    if !update_params.is_empty() {
        // Return error if found duplication of existing data detected
        if check_duplicated_params(&data.component_uuid, &update_params, conn)? {
            return Err(ServiceError::BadRequest(
                "Duplication of existing data detected".to_string()
            ))
        }

        count_changed_rows += update_component_params_values(
            &data.component_uuid,
            &update_params,
            conn
        )?;
    }

    Ok(count_changed_rows as i32)
}

/// Add new params from array InsertableComponentParam's
fn adding_new_component_params (
    data: &[InsertableComponentParam],
    conn: &PgConnection
) -> ServiceResult<usize> {
    diesel::insert_into(param_to_component)
        .values(data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail Inserted rows:  {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Update params values from array InsertableComponentParam's
/// for target component uuid
fn update_component_params_values(
    target_component_uuid: &Uuid,
    data: &[IptParamData],
    conn: &PgConnection
) -> ServiceResult<usize> {
    let mut res: usize = 0;

    for param_d in data {
        let insert_params = diesel::update(param_to_component
            .filter(component_uuid.eq(target_component_uuid)
            .and(param_id.eq(&param_d.param_id))))
            .set(value.eq(param_d.value.to_string()))
            .execute(conn)
            .map_err(|err| {
                debug!("Fail updated rows:  {:?}", err);
                ServiceError::InternalServerError
            })?;

        debug!("Updated {:?} rows", insert_params);
        res += insert_params;
    }

    Ok(res) // <-- return count
}

/// Find duplicate params data
/// return true if found duplication
fn check_duplicated_params(
    target_component_uuid: &Uuid,
    data: &[IptParamData],
    conn: &PgConnection
) -> ServiceResult<bool> {
    for param_d in data {
        let duplicate_params = param_to_component
            .filter(component_uuid.eq(target_component_uuid)
            .and(param_id.eq(&param_d.param_id)
            .and(value.eq(&param_d.value))))
            .execute(conn)
            .map_err(|err| {
                debug!("Fail updated rows:  {:?}", err);
                ServiceError::InternalServerError
            })?;

        if duplicate_params > 0 {
            debug!("Found {:?} duplicates rows", duplicate_params);
            return Ok(true)
        }
    }

    Ok(false) // <-- not found duplicates
}
