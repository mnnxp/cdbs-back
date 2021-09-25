use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::param::model::{
    IptComponentParamData,
    InsertableComponentParam
};
use crate::models::relate_ref::param::model::IptParamData;
use crate::schema::param_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Add new params with values for component
/// or update values an existing component params
pub(crate) fn put_component_params(
    data: &IptComponentParamData,
    conn: &PgConnection
) -> ServiceResult<i32> {
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
            match param_to_component
                .filter(component_uuid.eq(&data.component_uuid)
                .and(param_id.eq(&param_d.param_id)))
                .execute(conn) {
                Ok(ex) => {
                    if ex > 0 {
                        update_params.push(param_d.to_owned())     // <-- already has param need update
                    } else {
                        let insertable_data = InsertableComponentParam {
                            component_uuid: data.component_uuid,
                            param_id: param_d.param_id,
                            value: param_d.value.to_string(),
                        };
                        new_params.push(insertable_data)        // <-- need insert new param
                    }
                },
                Err(err) => {
                    debug!("Fail check param data: {:?} ", err);
                    return Err(ServiceError::BadRequest(
                        "Fail check param data".to_string()
                    ))
                },
            }
        }
    }

    // adding new params
    if !new_params.is_empty() {
        match adding_new_component_params(&new_params, conn) {
            x if x > 0 => count_changed_rows += x,
            _ => {
                return Err(ServiceError::BadRequest(
                    "Fail insert rows".to_string()
                ))
            },
        }
    }

    // updating params values
    if !update_params.is_empty() {
        // Return error if found duplication of existing data detected
        if check_duplicated_params(
            &data.component_uuid,
            &update_params,
            conn,
        ) {
            return Err(ServiceError::BadRequest(
                "Duplication of existing data detected".to_string()
            ))
        }

        match update_component_params_values(
            &data.component_uuid,
            &update_params,
            conn
        ) {
            x if x > 0 => count_changed_rows += x,
            _ => {
                return Err(ServiceError::BadRequest(
                    "Fail updated rows".to_string()
                ))
            },
        }
    }

    Ok(count_changed_rows as i32)
}

/// Add new params from array InsertableComponentParam's
fn adding_new_component_params (
    data: &[InsertableComponentParam],
    conn: &PgConnection
) -> usize {
    let insert_params = diesel::insert_into(param_to_component)
        .values(data)
        .execute(conn);

    match insert_params {
        Ok(count) => {
            debug!("Inserted {:?} rows", count);
            count   // <-- return count
        },
        Err(err) => {
            debug!("Fail Inserted rows:  {:?}", err);
            // return Err(ServiceError::BadRequest(
            //     "Fail insert rows".to_string()
            // ))
            0       // <-- return 0
        },
    }
}

/// Update params values from array InsertableComponentParam's
/// for target component uuid
fn update_component_params_values(
    target_component_uuid: &Uuid,
    data: &[IptParamData],
    conn: &PgConnection
) -> usize {
    let mut res: usize = 0;

    for param_d in data {
        let insert_params = diesel::update(param_to_component
            .filter(component_uuid.eq(target_component_uuid)
            .and(param_id.eq(&param_d.param_id))))
            .set(value.eq(param_d.value.to_string()))
            .execute(conn);

        match insert_params {
            Ok(count) => {
                debug!("Updated {:?} rows", count);
                res += count;
            },
            Err(err) => {
                debug!("Fail updated rows:  {:?}", err);
                // return Err(ServiceError::BadRequest(
                //     "Fail updated rows".to_string()
                // ))
            },
        }
    }

    res // <-- return count
}

/// Find duplicate params data
/// return true if found duplication
fn check_duplicated_params(
    target_component_uuid: &Uuid,
    data: &[IptParamData],
    conn: &PgConnection
) -> bool {
    for param_d in data {
        let duplicate_params = param_to_component
            .filter(component_uuid.eq(target_component_uuid)
            .and(param_id.eq(&param_d.param_id)
            .and(value.eq(&param_d.value))))
            .execute(conn);

        match duplicate_params {
            Ok(count) if count > 0 => {
                debug!("Found {:?} duplicates rows", count);
                return true
            },
            Ok(_) => (), // <-- just next
            Err(err) => {
                debug!("Fail seatch duplicates rows:  {:?}", err);
            },
        }
    }

    false // <-- not found duplicates
}
