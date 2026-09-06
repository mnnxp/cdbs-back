use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::graphql::component_model::IptUpdateComponentData;
use crate::schema::component_modification_list::dsl as component_modification_list;
use crate::schema::component_ref::dsl as component_ref;
use chrono::Utc;
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
    // update data validation
    if data
        .description
        .as_ref()
        .map(|d| d.chars().count())
        .unwrap_or_default()
        > 50000
    {
        return Err(get_err_msg(ErrorMessage::TextMustLess(50000)));
    }

    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        target_component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column parent_component_uuid
    if let Some(value) = &data.parent_component_uuid {
        count_update_columns += diesel::update(
            component_ref::component_ref.filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::parent_component_uuid.ne(value)),
            ),
        )
        .set(component_ref::parent_component_uuid.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column name
    if let Some(value) = &data.name {
        count_update_columns += diesel::update(
            component_ref::component_ref.filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::name.ne(value)),
            ),
        )
        .set(component_ref::name.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(
            component_ref::component_ref.filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::description.ne(value)),
            ),
        )
        .set(component_ref::description.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column component_type_id
    if let Some(value) = &data.component_type_id {
        count_update_columns += diesel::update(
            component_ref::component_ref.filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::component_type_id.ne(value)),
            ),
        )
        .set(component_ref::component_type_id.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column actual_status_id
    if let Some(value) = &data.actual_status_id {
        count_update_columns += diesel::update(
            component_ref::component_ref.filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::actual_status_id.ne(value)),
            ),
        )
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

    change_updated_at(target_component_uuid, None, conn)?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}

/// Sets current time as value updated at for target component and modification (optional)
pub(crate) fn change_updated_at(
    target_component_uuid: &Uuid,
    target_modification_uuid: Option<&Uuid>,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let new_updated_at = Utc::now().naive_utc();
    let res = diesel::update(
        component_ref::component_ref.filter(component_ref::uuid.eq(target_component_uuid)),
    )
    .set(component_ref::updated_at.eq(new_updated_at))
    .execute(conn)
    .map_err(|err| {
        debug!("Failed update data: {:?}", err);
        get_err_msg(ErrorMessage::FailedUpdateData)
    })?;
    if let Some(tmu) = target_modification_uuid {
        diesel::update(
            component_modification_list::component_modification_list
                .filter(component_modification_list::uuid.eq(tmu)),
        )
        .set(component_modification_list::updated_at.eq(new_updated_at))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }
    Ok(res)
}
