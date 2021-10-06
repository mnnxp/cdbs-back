use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::user::model::{
    UserAccessStandard,
    UserAccessStandardAndRelatedData,
    IptUserAccessStandardData,
    InsertableUserAccessStandard,
    DelUserAccessStandardData,
};
use crate::models::standard::access::util::check_is_owner;
use crate::schema::user_access_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Get users list have access to standard
pub(crate) fn get_users_list_access_standard(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<UserAccessStandardAndRelatedData>> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_user_uuid, target_standard_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. получить список пользователей с доступом к компоненту
    let list_users_with_access = UserAccessStandardAndRelatedData::from_standard_by_uuid(
        target_standard_uuid,
        set_lang_id,
        conn
    );

    match list_users_with_access {
        Ok(res) => {
            debug!("Get users have access: {:?}", res);
            Ok(res)
        },
        Err(err) => {
            debug!("Failed get users list have access to standard: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed get users list have access to standard".to_string()
            ))
        },
    }
}

/// Manage standard access for user
pub(crate) fn set_user_access_standard(
    logged_user_uuid: &Uuid,
    data: &IptUserAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_user_uuid, &data.standard_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. изменить или добавить доступ для указанного пользователя
    let set_access = diesel::update(user_access_to_standard
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid))))
        .set((
            type_access_id.eq(data.type_access_id),
            is_enabled.eq(true),
            updated_at.eq(chrono::Local::now().naive_local())
        )).execute(conn);

    match set_access {
        Ok(0) => {
            // доступ не найден, добавить новую запись
            if add_user_access_standard(
                data,
                conn
            )? { return Ok(true) }

            Err(ServiceError::BadRequest(
                "Failed set access for target user".to_string()
            ))
        },
        Ok(x) => {
            debug!("Set access for target user: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed set access for target user: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed set access for target user".to_string()
            ))
        },
    }
}

/// Add new access standard for user
/// Warning: this function without "check is owner user"
fn add_user_access_standard(
    data: &IptUserAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableUserAccessStandard = data.into();

    let add_new_access: Result<UserAccessStandard, diesel::result::Error> =
        diesel::insert_into(user_access_to_standard)
            .values(insert_data)
            .get_result(conn);

    match add_new_access {
        Ok(x) => {
            debug!("Completed add new access for target user: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed add access for target user: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed add access for target user".to_string()
            ))
        },
    }
}

/// Remove access standard for user
pub(crate) fn del_user_access_standard(
    logged_user_uuid: &Uuid,
    data: &DelUserAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_user_uuid, &data.standard_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. деактивировать доступ для указанного пользователя
    let del_access = diesel::delete(user_access_to_standard)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .execute(conn);

    match del_access {
        Ok(0) => {
            // доступ не найден
            Err(ServiceError::BadRequest(
                "Access not found for user".to_string()
            ))
        },
        Ok(x) => {
            debug!("Delete access for target user: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed delete access for target user: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed delete access for target user".to_string()
            ))
        },
    }
}
