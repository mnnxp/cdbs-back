use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::graphql::component_model::IptUpdateComponentData;
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет основные данные компонента по UUID.
/// Возвращает количество успешных изменений или ошибку, если все указанные данные уже существуют.
pub(crate) fn update_component_by_uuid(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    data: &IptUpdateComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // need top level access for change component main data
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        target_component_uuid,
        &need_access_level,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column parent_component_uuid
    if let Some(value) = &data.parent_component_uuid {
        count_update_columns += diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::parent_component_uuid.ne(value))))
            .set(component_ref::parent_component_uuid.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column name
    if let Some(value) = &data.name {
        count_update_columns += diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::name.ne(value))))
            .set(component_ref::name.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::description.ne(value))))
            .set(component_ref::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column component_type_id
    if let Some(value) = &data.component_type_id {
        count_update_columns += diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::component_type_id.ne(value))))
            .set(component_ref::component_type_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column actual_status_id
    if let Some(value) = &data.actual_status_id {
        count_update_columns += diesel::update(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::actual_status_id.ne(value))))
            .set(component_ref::actual_status_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(get_err_msg(ErrorMessage::DataHasAlready));
    }

    diesel::update(component_ref::component_ref
        .filter(component_ref::uuid.eq(target_component_uuid)))
        .set(component_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
