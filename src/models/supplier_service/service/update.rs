use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::graphql::service_model::{IptServiceStatusArg, IptUpdateServiceData};
use crate::models::supplier_service::access::util::{check_is_owner_with_err, check_user_access_provided_by_company, get_service_status};
use crate::models::search::model::ExtraOptions;
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use chrono::Local;
use uuid::Uuid;

/// Updates the service master data by UUID.
/// Returns the number of successful changes or an error if all specified data already exists.
pub(crate) fn update_service_data(
    target_service_uuid: &Uuid,
    data: &IptUpdateServiceData,
    options: &ExtraOptions,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    // update data validation
    if get_service_status(target_service_uuid, conn)? > 1 {
        return Err(get_err_msg(ErrorMessage::FailedUpdateServiceBadStatus))
    }
    if data.description.as_ref().map(|d| d.len()).unwrap_or_default() > 2000 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(2000)));
    }

    check_is_owner_with_err(
        &options.logged_user_uuid,
        target_service_uuid,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column name
    if let Some(value) = &data.name {
        count_update_columns += diesel::update(service_ref::service_ref
            .filter(service_ref::uuid.eq(target_service_uuid)
            .and(service_ref::name.ne(value))))
            .set(service_ref::name.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(service_ref::service_ref
            .filter(service_ref::uuid.eq(target_service_uuid)
            .and(service_ref::description.ne(value))))
            .set(service_ref::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(service_ref::service_ref
            .filter(service_ref::uuid.eq(target_service_uuid)
            .and(service_ref::region_id.ne(value))))
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

    diesel::update(service_ref::service_ref
        .filter(service_ref::uuid.eq(target_service_uuid)))
        .set(service_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    debug!("Count update columns: {:?}", count_update_columns);
    change_updated_at(target_service_uuid, conn)?;
    Ok(count_update_columns)
}

/// Changes of service status. Available only to members of the provider's company.
pub(crate) fn change_service_status(
    args: &IptServiceStatusArg,
    options: &ExtraOptions,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    let need_access_level = 2; // todo!(create enum for manage access level)
    // checking the availability of user access provided by the company
    check_user_access_provided_by_company(
        &options.logged_user_uuid,
        &args.service_uuid,
        &need_access_level,
        conn
    )?;
    // update column service_status_id
    let count_update_columns = diesel::update(service_ref::service_ref
        .filter(service_ref::uuid.eq(&args.service_uuid)
        .and(service_ref::service_status_id.ne(&args.service_status_id))))
        .set(service_ref::service_status_id.eq(&args.service_status_id))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    if count_update_columns > 0 {
        change_updated_at(&args.service_uuid, conn)?;
    }
    Ok(count_update_columns == 1)
}

/// Sets current time as value updated at for target service and modification (optional)
pub(crate) fn change_updated_at(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    diesel::update(service_ref::service_ref
        .filter(service_ref::uuid.eq(target_service_uuid)))
        .set(service_ref::updated_at.eq(Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })
}