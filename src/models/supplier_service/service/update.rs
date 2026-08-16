use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::graphql::service_model::{IptServiceStatusArg, IptUpdateServiceData};
use crate::models::search::model::ExtraOptions;
use crate::models::supplier_service::access::util::check_is_owner_with_err;
use crate::models::supplier_service::history::save_log_service_change;
use crate::models::supplier_service::util::{get_service_consumer, get_service_status};
use crate::models::user::notification::model::{NotificationData, NotificationType};
use crate::models::user::notification::service::register::create_notification;
use crate::schema::service_ref::dsl as service_ref;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

/// Updates the service master data by UUID.
/// Returns the number of successful changes or an error if all specified data already exists.
pub(crate) fn update_service_data(
    target_service_uuid: &Uuid,
    data: &IptUpdateServiceData,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // update data validation
    if get_service_status(target_service_uuid, conn)? > 3 {
        return Err(get_err_msg(ErrorMessage::FailedUpdateServiceBadStatus));
    }
    if data
        .description
        .as_ref()
        .map(|d| d.chars().count())
        .unwrap_or_default()
        > 50000
    {
        return Err(get_err_msg(ErrorMessage::TextMustLess(50000)));
    }

    check_is_owner_with_err(&options.logged_user_uuid, target_service_uuid, conn)?;

    // for returning change count
    let mut count_update_columns = 0_usize;
    let mut old_name: Option<String> = None;
    let mut old_description: Option<String> = None;
    let mut old_region_id: Option<i32> = None;

    // update column name
    if let Some(value) = &data.name {
        old_name = service_ref::service_ref
            .filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::name.ne(value)),
            )
            .select(service_ref::name)
            .first(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckData)
            })?;
        count_update_columns += diesel::update(
            service_ref::service_ref.filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::name.ne(value)),
            ),
        )
        .set(service_ref::name.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column description
    if let Some(value) = &data.description {
        old_description = service_ref::service_ref
            .filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::description.ne(value)),
            )
            .select(service_ref::description)
            .first(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckData)
            })?;
        count_update_columns += diesel::update(
            service_ref::service_ref.filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::description.ne(value)),
            ),
        )
        .set(service_ref::description.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        old_region_id = service_ref::service_ref
            .filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::region_id.ne(value)),
            )
            .select(service_ref::region_id)
            .first(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckData)
            })?;
        count_update_columns += diesel::update(
            service_ref::service_ref.filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::region_id.ne(value)),
            ),
        )
        .set(service_ref::region_id.eq(value))
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
    debug!("Count update columns: {:?}", count_update_columns);
    change_service_updated_at(
        target_service_uuid,
        &options.logged_user_uuid,
        format!("Modified name {old_name:?}, description {old_description:?}, region_id {old_region_id:?}"),
        conn,
    )?;
    // add notification for user
    create_notification(
        &options.logged_user_uuid,
        &NotificationData {
            notification: format!("Service UUID:{target_service_uuid} has been updated by creator"),
            degree_importance: NotificationType::Info,
        },
        conn,
    )?;
    Ok(count_update_columns)
}

/// Changes of service status. Available only to members of the provider's company.
pub(crate) fn change_service_status(
    args: &IptServiceStatusArg,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // update data validation
    if get_service_status(&args.service_uuid, conn)? > 9 {
        return Err(get_err_msg(ErrorMessage::FailedUpdateServiceBadStatus));
    }
    require_permission(
        &options.logged_user_uuid,
        AccessEntity::Service,
        &args.service_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let old_service_status_id = service_ref::service_ref
        .filter(service_ref::uuid.eq(&args.service_uuid))
        .select(service_ref::service_status_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get old data: {:?}", err);
            get_err_msg(ErrorMessage::FailedCheckData)
        })?;
    // update column service_status_id
    let count_update_columns = diesel::update(
        service_ref::service_ref.filter(
            service_ref::uuid
                .eq(&args.service_uuid)
                .and(service_ref::service_status_id.ne(&args.service_status_id)),
        ),
    )
    .set(service_ref::service_status_id.eq(&args.service_status_id))
    .execute(conn)
    .map_err(|err| {
        debug!("Failed update data: {:?}", err);
        get_err_msg(ErrorMessage::FailedUpdateData)
    })?;
    if count_update_columns == 0 {
        return Ok(false);
    }
    change_service_updated_at(
        &args.service_uuid,
        &options.logged_user_uuid,
        format!("Changed status, old ID:{old_service_status_id}"),
        conn,
    )?;
    // add notification for user
    create_notification(
        &get_service_consumer(&args.service_uuid, conn)?,
        &NotificationData {
            notification: format!(
                "Status of the UUID:{} service has been changed",
                &args.service_uuid
            ),
            degree_importance: NotificationType::Info,
        },
        conn,
    )?;
    Ok(true)
}

/// Sets current time as value updated at for target service and modification (optional)
pub(crate) fn change_service_updated_at(
    target_service_uuid: &Uuid,
    logged_user_uuid: &Uuid,
    old_data: String,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    save_log_service_change(target_service_uuid, logged_user_uuid, old_data, conn);
    diesel::update(service_ref::service_ref.filter(service_ref::uuid.eq(target_service_uuid)))
        .set(service_ref::updated_at.eq(Utc::now().naive_utc()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })
}
