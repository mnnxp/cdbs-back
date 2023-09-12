use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_modification::param::model::{
    IptModificationParamData, InsertableModificationParam
};
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::relate_ref::param::model::IptParamData;
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::param_to_modification::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет новые параметры со значениями для модификации компонента.
/// Обновляет значения существующих параметров модификации компонента, если предоставленные имена параметров уже существуют.
pub(crate) fn put_modification_params(
    logged_user_uuid: &Uuid,
    data: &IptModificationParamData,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&data.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    if data.params.is_empty() {
        return Err(ServiceError::BadRequest(
            "Not found params for adding or updating".to_string()
        ))
    }

    let mut count_changed_rows: usize = 0;

    let mut new_params: Vec<InsertableModificationParam> = Vec::new();      // <-- new parameters to be added
    let mut update_params: Vec<IptParamData> = Vec::new();   // <-- found parameters will be updated

    for param_d in &data.params {
        if param_d.param_id > 0 { // <-- additionally we check the correctness of the id
            let get_param = param_to_modification
                .filter(modification_uuid.eq(&data.modification_uuid)
                .and(param_id.eq(&param_d.param_id)))
                .execute(conn)
                .map_err(|err| {
                    debug!("Fail check param data: {:?} ", err);
                    ServiceError::InternalServerError
                })?;

            match get_param {
                0 => {
                    let insertable_data = InsertableModificationParam {
                        modification_uuid: data.modification_uuid,
                        param_id: param_d.param_id,
                        value: param_d.value.to_string(),
                    };
                    new_params.push(insertable_data);        // <-- need insert new param
                },
                _ => update_params.push(param_d.clone()),     // <-- already has param need update
            }
        }
    }

    // adding new params
    if !new_params.is_empty() {
        count_changed_rows += adding_new_modification_params(&new_params, conn)?;
    }

    // updating params values
    if !update_params.is_empty() {
        // Return error if found duplication of existing data detected
        if check_duplicated_params(&data.modification_uuid, &update_params, conn)? {
            return Err(ServiceError::BadRequest(
                "Duplication of existing data detected".to_string()
            ))
        }

        count_changed_rows += update_modification_params_values(
            &data.modification_uuid,
            &update_params,
            conn
        )?;
    }

    Ok(count_changed_rows as i32)
}

/// Add new params from array InsertableModificationParam's
fn adding_new_modification_params (
    data: &[InsertableModificationParam],
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    diesel::insert_into(param_to_modification)
        .values(data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail Inserted rows:  {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Update params values from array InsertableModificationParam's
/// for target component modification uuid
fn update_modification_params_values(
    target_modification_uuid: &Uuid,
    data: &[IptParamData],
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let mut res: usize = 0;

    for param_d in data {
        let insert_params = diesel::update(param_to_modification
            .filter(modification_uuid.eq(target_modification_uuid)
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
    target_modification_uuid: &Uuid,
    data: &[IptParamData],
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    for param_d in data {
        let duplicate_params = param_to_modification
            .filter(modification_uuid.eq(target_modification_uuid)
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
