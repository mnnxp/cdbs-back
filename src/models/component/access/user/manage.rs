use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::access::user::model::{
    UserAccessComponentAndRelatedData,
    IptUserAccessComponentData,
    InsertableUserAccessComponent,
    DelUserAccessComponentData,
};
use crate::models::component::access::util::check_is_owner_with_err;
use crate::models::search::model::ExtraOptions;
use crate::schema::user_access_to_component::dsl as user_access_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список пользователей, имеющих доступ к компоненту.
pub(crate) fn get_users_list_access_component(
    target_component_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(
        &options.logged_user_uuid,
        target_component_uuid,
        conn
    )?;

    // 2. получить список пользователей с доступом к компоненту
    UserAccessComponentAndRelatedData::from_component_by_uuid(
        target_component_uuid,
        &options.set_lang_id,
        conn
    )
}

/// Устанавливает доступ к компоненту для пользователя.
pub(crate) fn set_user_access_component(
    logged_user_uuid: &Uuid,
    data: &IptUserAccessComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.component_uuid, conn)?;

    let get_access = user_access_to_component::user_access_to_component
        .filter(user_access_to_component::component_uuid.eq(&data.component_uuid)
        .and(user_access_to_component::user_uuid.eq(&data.user_uuid)))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed ready access: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_access {
        1 => {
            // 2. изменить доступ для указанного пользователя
            diesel::update(user_access_to_component::user_access_to_component
                .filter(user_access_to_component::component_uuid.eq(&data.component_uuid)
                .and(user_access_to_component::user_uuid.eq(&data.user_uuid))))
                .set((
                    user_access_to_component::type_access_id.eq(data.type_access_id),
                    user_access_to_component::is_enabled.eq(true),
                    user_access_to_component::updated_at.eq(chrono::Local::now().naive_local())
                ))
                .returning(user_access_to_component::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed set user access: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        // доступ не найден, добавить новую запись
        _ => add_user_access_component(data, conn),
    }
}

/// Add new access component for user
/// Warning: this function without "check is owner user"
fn add_user_access_component(
    data: &IptUserAccessComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableUserAccessComponent = data.into();

    diesel::insert_into(user_access_to_component::user_access_to_component)
        .values(&insert_data)
        .returning(user_access_to_component::is_enabled)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed add access for target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Удаляет доступ к компоненту для пользователя.
pub(crate) fn del_user_access_component(
    logged_user_uuid: &Uuid,
    data: &DelUserAccessComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.component_uuid, conn)?;

    // 2. деактивировать доступ для указанного пользователя
    let del_access = diesel::delete(user_access_to_component::user_access_to_component)
        .filter(user_access_to_component::component_uuid.eq(&data.component_uuid)
        .and(user_access_to_component::user_uuid.eq(&data.user_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete access for target user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match del_access {
        1 => Ok(true),
        // доступ не найден
        _ => Err(get_err_msg(ErrorMessage::AccessNotFoundUser)),
    }
}
