use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::service_model::IptServiceData;
use crate::models::supplier_service::model::InsertableService;
use crate::models::company::util::check_is_supplier;
use crate::models::search::model::ExtraOptions;
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Creates a service request for the company, the company must have supplier status.
/// An authorized user becomes the owner of the service request. Returns the UUID of the new service.
pub(crate) fn create_service(
    data: &IptServiceData,
    options: &ExtraOptions,
    conn: &mut PgConnection
) -> ServiceResult<Uuid> {
    check_is_supplier(&data.company_uuid, conn)?;

    let insert_data = InsertableService::by_args(data, &options.logged_user_uuid);

    diesel::insert_into(service_ref::service_ref)
        .values(&insert_data)
        .returning(service_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed created service: {:?}", err);
            ServiceError::InternalServerError
        })
}
