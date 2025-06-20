use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::model::ExtraOptions;
use crate::models::supplier_service::access::user::model::{
    DelUserAccessServiceData, InsertableUserAccessService, IptUserAccessServiceData,
    UserAccessServiceAndRelatedData,
};
use crate::models::supplier_service::access::util::check_is_owner_with_err;
use crate::schema::user_access_to_service::dsl as user_access_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the list of users who have access to the service
pub(crate) fn get_users_list_access_service(
    target_service_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserAccessServiceAndRelatedData>> {
    // 1. check the user's ownership of the component
    check_is_owner_with_err(&options.logged_user_uuid, target_service_uuid, conn)?;

    // 2. get the list of users with access to the component
    UserAccessServiceAndRelatedData::from_service_by_uuid(
        target_service_uuid,
        &options.set_lang_id,
        conn,
    )
}

///  Sets the user's access to the service
pub(crate) fn set_user_access_service(
    data: &IptUserAccessServiceData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. check the user's ownership of the component
    check_is_owner_with_err(logged_user_uuid, &data.service_uuid, conn)?;

    // 2. change or add access for a specified user
    let get_access = user_access_to_service::user_access_to_service
        .filter(
            user_access_to_service::service_uuid
                .eq(&data.service_uuid)
                .and(user_access_to_service::user_uuid.eq(&data.user_uuid)),
        )
        .select(user_access_to_service::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed change access for user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_access.first() {
        Some(_) => diesel::update(
            user_access_to_service::user_access_to_service.filter(
                user_access_to_service::service_uuid
                    .eq(&data.service_uuid)
                    .and(user_access_to_service::user_uuid.eq(&data.user_uuid)),
            ),
        )
        .set((
            user_access_to_service::type_access_id.eq(data.type_access_id),
            user_access_to_service::is_enabled.eq(true),
            user_access_to_service::updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .returning(user_access_to_service::is_enabled)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed change access for user: {:?}", err);
            ServiceError::InternalServerError
        }),
        None => add_user_access_service(data, conn),
    }
}

/// Add new access service for user
/// Warning: this function without "check is owner user"
fn add_user_access_service(
    data: &IptUserAccessServiceData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableUserAccessService = data.into();

    diesel::insert_into(user_access_to_service::user_access_to_service)
        .values(&insert_data)
        .returning(user_access_to_service::is_enabled)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed delete access for target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Removes access to the service for the user
pub(crate) fn del_user_access_service(
    data: &DelUserAccessServiceData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. check the user's ownership of the component
    check_is_owner_with_err(logged_user_uuid, &data.service_uuid, conn)?;

    // 2. deactivate access for the specified user
    let del_access = diesel::delete(user_access_to_service::user_access_to_service)
        .filter(
            user_access_to_service::service_uuid
                .eq(&data.service_uuid)
                .and(user_access_to_service::user_uuid.eq(&data.user_uuid)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete access for target user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match del_access {
        1 => Ok(true),
        _ => Err(get_err_msg(ErrorMessage::AccessNotFoundUser)),
    }
}
