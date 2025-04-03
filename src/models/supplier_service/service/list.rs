use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::graphql::service_model::{ServiceAndRelatedData, ShowServiceShort};
use crate::models::supplier_service::model::ServicesArg;
use crate::models::search::{model::ExtraOptions, order::{Paginate, Sort}};
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Returns aggregated services data. Gets a summary of services filtered by:
/// UUID, company, user, favorites (for yourself or another user).
pub(crate) fn get_services(
    args: &ServicesArg,
    options: &ExtraOptions,
    sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowServiceShort>> {
    // collect services uuids for check access
    let target_services_uuids: Vec<Uuid> =
        match args.company_uuid.is_some() || args.user_uuid.is_some() {
            // gets services of user list with/without filter
            true => get_services_with_filter(args, paginate, conn)?,
            // get all public services
            false => args.filter_services_uuids.to_vec(),
        };

    // return not found if set search favorite and no favorite services
    if target_services_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowServiceShort::get_services(&target_services_uuids, options, sort, paginate, conn)
        .map_err(|err| {
            debug!("Error loading list services and collect short data: {:?}", err);
            get_err_msg(ErrorMessage::AccessDenied)
        })
}

/// Gets list with uuids services by owner user and/or company filter
fn get_services_with_filter(
    args: &ServicesArg,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::service_ref::dsl as service_ref;

    let mut query = service_ref::service_ref.into_boxed();
    query = match (args.filter_services_uuids.is_empty(), args.company_uuid, args.user_uuid) {
        (true, Some(company_uuid), None) => query.filter(service_ref::company_uuid.eq(company_uuid)),
        (true, None, Some(user_uuid)) => query.filter(service_ref::user_uuid.eq(user_uuid)),
        (false, Some(company_uuid), None) => {
            query.filter(service_ref::company_uuid.eq(company_uuid)
                .and(service_ref::uuid.eq_any(&args.filter_services_uuids)))
        },
        (false, None, Some(user_uuid)) => {
            query.filter(service_ref::user_uuid.eq(user_uuid)
                .and(service_ref::uuid.eq_any(&args.filter_services_uuids)))
        },
        (false, Some(company_uuid), Some(user_uuid)) => {
            query.filter(service_ref::company_uuid.eq(company_uuid)
                .and(service_ref::user_uuid.eq(user_uuid))
                .and(service_ref::uuid.eq_any(&args.filter_services_uuids)))
        },
        _ => return Err(get_err_msg(ErrorMessage::FailedMatchArguments)),
    };

    query.select(service_ref::uuid)
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Returns full information about the service by UUID
pub(crate) fn find_by_uuid(
    target_service_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<ServiceAndRelatedData> {
    // collect data for service
    let result: ServiceAndRelatedData = ServiceAndRelatedData::collect_related_data(
        target_service_uuid,
        options,
        conn
    ).expect("Error loading service and collect related data");
    debug!("Service data: {:#?}", result);
    Ok(result)
}
