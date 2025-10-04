use crate::errors::{ServiceError, ServiceResult};
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Gets service status id for target service
pub(crate) fn get_service_status(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    service_ref::service_ref
        .filter(service_ref::uuid.eq(target_service_uuid))
        .select(service_ref::service_status_id)
        .first::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Returns the consumer (user) Uuid for the service
pub(crate) fn get_service_consumer(
    service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    service_ref::service_ref
        .filter(service_ref::uuid.eq(service_uuid))
        .select(service_ref::user_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Not found consumer for service: {:?}", err);
            ServiceError::InternalServerError
        })
}
