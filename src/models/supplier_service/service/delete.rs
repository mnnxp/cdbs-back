use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::{file_to_service::dsl as file_to_service, service_ref::dsl as service_ref};
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;

/// Deletes the service and its associated files.
/// Deleting a service requires being both the owner of the service and the owner of the company.
/// Returns the UUID of the deleted service.
pub(crate) fn del_service_data(
    del_service_uuid: &Uuid,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // check ownership service
    check_is_owner_with_err(logged_user_uuid, del_service_uuid, conn)?;

    // if user is ownership the service, next check owner company
    let (owner_company_uuid, service_status_id) = service_ref::service_ref
        .filter(service_ref::uuid.eq(del_service_uuid))
        .select((service_ref::company_uuid, service_ref::service_status_id))
        .first::<(Uuid, i32)>(conn)
        .map_err(|err| {
            debug!("Not found service: {:?}", err);
            get_err_msg(ErrorMessage::NotFoundService)
        })?;

    // deletes if owner of both services and company or if owner and services, but only created
    if service_status_id > 1 {
        // check ownership company of the service
        check_is_owner_with_err(logged_user_uuid, &owner_company_uuid, conn)?;
    }
    delete_service_files(del_service_uuid, conn)?;
    delete_service(del_service_uuid, conn)?;
    Ok(*del_service_uuid)
}

/// Set the delete flags for all files associated with the service
fn delete_service_files(del_service_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let del_file_uuids = file_to_service::file_to_service
        .filter(file_to_service::service_uuid.eq(del_service_uuid))
        .select(file_to_service::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of service: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}

/// Set the delete flags for the service
fn delete_service(del_service_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<usize> {
    diesel::update(service_ref::service_ref)
        .filter(service_ref::uuid.eq(del_service_uuid))
        .set((
            service_ref::is_delete.eq(true),
            service_ref::updated_at.eq(Local::now().naive_local()),
        ))
        .execute(conn)
        .map_err(|err| {
            debug!("Failure when set is_delete flag for a service: {:?}", err);
            ServiceError::InternalServerError
        })
}
