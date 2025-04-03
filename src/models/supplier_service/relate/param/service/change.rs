use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::supplier_service::service::update::change_updated_at;
use crate::models::supplier_service::{
    param::model::{IptServiceParamsData, InsertableServiceParam},
    access::util::check_access_service_for_user,
};
use crate::models::relate_ref::param::model::IptParamData;
use crate::models::search::model::ExtraOptions;
use crate::schema::param_to_service::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет новые параметры со значениями для компонента.
/// Обновляет значения существующих параметров компонента, если предоставленные имена параметров уже существуют.
pub(crate) fn put_service_params(
    data: &IptServiceParamsData,
    options: &ExtraOptions,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        &options.logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn
    )?;

    if data.params.is_empty() {
        return Err(get_err_msg(ErrorMessage::NotFoundParamsForAddingOrUpdaing))
    }

    let mut count_changed_rows: usize = 0;

    let mut new_params: Vec<InsertableServiceParam> = Vec::new();      // <-- new parameters to be added
    let mut update_params: Vec<IptParamData> = Vec::new();   // <-- found parameters will be updated

    for param_d in &data.params {
        if param_d.param_id > 0 { // <-- additionally we check the correctness of the id
            let get_param = param_to_service
                .filter(service_uuid.eq(&data.service_uuid)
                .and(param_id.eq(&param_d.param_id)))
                .execute(conn)
                .map_err(|err| {
                    debug!("Fail check param data: {:?} ", err);
                    ServiceError::InternalServerError
                })?;

            match get_param {
                0 => {
                    let insertable_data = InsertableServiceParam {
                        service_uuid: data.service_uuid,
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
        count_changed_rows += adding_new_service_params(&new_params, conn)?;
    }

    // updating params values
    if !update_params.is_empty() {
        // Return error if found duplication of existing data detected
        if check_duplicated_params(&data.service_uuid, &update_params, conn)? {
            return Err(get_err_msg(ErrorMessage::DuplicateOfExistingData))
        }

        count_changed_rows += update_service_params_values(
            &data.service_uuid,
            &update_params,
            conn
        )?;

        change_updated_at(&data.service_uuid, conn)?;
    }

    Ok(count_changed_rows)
}

/// Add new params from array InsertableServiceParam's
fn adding_new_service_params (
    data: &[InsertableServiceParam],
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    diesel::insert_into(param_to_service)
        .values(data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail Inserted rows:  {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Update params values from array InsertableServiceParam's
/// for target service uuid
fn update_service_params_values(
    target_service_uuid: &Uuid,
    data: &[IptParamData],
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let mut res: usize = 0;

    for param_d in data {
        let insert_params = diesel::update(param_to_service
            .filter(service_uuid.eq(target_service_uuid)
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
    target_service_uuid: &Uuid,
    data: &[IptParamData],
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    for param_d in data {
        let duplicate_params = param_to_service
            .filter(service_uuid.eq(target_service_uuid)
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
