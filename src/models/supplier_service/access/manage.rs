use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::model::ExtraOptions;
use crate::models::supplier_service::access::model::{ChangeOwnerService, ChangeTypeAccessService};
use crate::models::supplier_service::access::util::check_is_owner_with_err;
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Transfer of ownership of the service to another user
pub(crate) fn change_service_owner_user(
    data: &ChangeOwnerService,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. verify the user's possession of the service
    check_is_owner_with_err(&options.logged_user_uuid, &data.service_uuid, conn)?;

    // 2. change the service owner
    let change_owner = diesel::update(
        service_ref::service_ref.filter(
            service_ref::uuid
                .eq(&data.service_uuid)
                .and(service_ref::user_uuid.eq(&options.logged_user_uuid)),
        ),
    )
    .set(service_ref::user_uuid.eq(&data.new_owner_user_uuid))
    .returning(service_ref::user_uuid)
    .get_result::<Uuid>(conn)
    .map_err(|err| {
        debug!("Failed change owner service: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_owner == data.new_owner_user_uuid)
}

/// Changes the default access to the service.
pub(crate) fn change_service_type_access(
    data: &ChangeTypeAccessService,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. verify the user's possession of the service
    check_is_owner_with_err(&options.logged_user_uuid, &data.service_uuid, conn)?;

    // 2. change the access type of the service
    let change_access = diesel::update(
        service_ref::service_ref.filter(
            service_ref::uuid
                .eq(&data.service_uuid)
                .and(service_ref::user_uuid.eq(&options.logged_user_uuid)),
        ),
    )
    .set(service_ref::type_access_id.eq(&data.new_type_access_id))
    .returning(service_ref::type_access_id)
    .get_result::<i32>(conn)
    .map_err(|err| {
        debug!("Failed change access service: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_access == data.new_type_access_id)
}
