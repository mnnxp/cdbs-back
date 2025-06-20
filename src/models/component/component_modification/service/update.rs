use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::model::IptUpdateComponentModificationData;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::component::service::update::change_updated_at;
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
    // update data validation
    if data
        .description
        .as_ref()
        .map(|d| d.len())
        .unwrap_or_default()
        > 2000
    {
        return Err(get_err_msg(ErrorMessage::TextMustLess(2000)));
    }

    let need_access_level = 1; // todo!(create enum for manage access level)
    let target_component_uuid = get_component_by_modification(target_modification_uuid, conn)?;
    check_access_component_for_user(
        logged_user_uuid,
        &target_component_uuid,
        &need_access_level,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column modification_name
    if let Some(value) = &data.modification_name {
        count_update_columns += diesel::update(
            component_modification_list::component_modification_list.filter(
                component_modification_list::uuid
                    .eq(target_modification_uuid)
                    .and(component_modification_list::modification_name.ne(value)),
            ),
        )
        .set(component_modification_list::modification_name.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(
            component_modification_list::component_modification_list.filter(
                component_modification_list::uuid
                    .eq(target_modification_uuid)
                    .and(component_modification_list::description.ne(value)),
            ),
        )
        .set(component_modification_list::description.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column actual_status_id
    if let Some(value) = &data.actual_status_id {
        count_update_columns += diesel::update(
            component_modification_list::component_modification_list.filter(
                component_modification_list::uuid
                    .eq(target_modification_uuid)
                    .and(component_modification_list::actual_status_id.ne(value)),
            ),
        )
        .set(component_modification_list::actual_status_id.eq(value))
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

    // update the updated_at of component and modification if modification are updated
    if count_update_columns > 0 {
        change_updated_at(&target_component_uuid, Some(target_modification_uuid), conn)?;
    }
    debug!("Count update columns: {:?}", count_update_columns);
    Ok(count_update_columns)
}
