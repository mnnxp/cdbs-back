use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::user::model::{
    DelUserAccessStandardData, InsertableUserAccessStandard, IptUserAccessStandardData,
    UserAccessStandardAndRelatedData,
};
use crate::models::standard::access::util::check_is_owner_with_err;
use crate::schema::user_access_to_standard::dsl as user_access_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список пользователей, имеющих доступ к стандарту.
pub(crate) fn get_users_list_access_standard(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UserAccessStandardAndRelatedData>> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, target_standard_uuid, conn)?;

    // 2. получить список пользователей с доступом к компоненту
    UserAccessStandardAndRelatedData::from_standard_by_uuid(target_standard_uuid, set_lang_id, conn)
}

/// Устанавливает доступ к стандарту для пользователя.
pub(crate) fn set_user_access_standard(
    logged_user_uuid: &Uuid,
    data: &IptUserAccessStandardData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. изменить или добавить доступ для указанного пользователя
    let get_access = user_access_to_standard::user_access_to_standard
        .filter(
            user_access_to_standard::standard_uuid
                .eq(&data.standard_uuid)
                .and(user_access_to_standard::user_uuid.eq(&data.user_uuid)),
        )
        .select(user_access_to_standard::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed change access for user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_access.first() {
        Some(_) => diesel::update(
            user_access_to_standard::user_access_to_standard.filter(
                user_access_to_standard::standard_uuid
                    .eq(&data.standard_uuid)
                    .and(user_access_to_standard::user_uuid.eq(&data.user_uuid)),
            ),
        )
        .set((
            user_access_to_standard::type_access_id.eq(data.type_access_id),
            user_access_to_standard::is_enabled.eq(true),
            user_access_to_standard::updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .returning(user_access_to_standard::is_enabled)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed change access for user: {:?}", err);
            ServiceError::InternalServerError
        }),
        None => add_user_access_standard(data, conn),
    }
}

/// Add new access standard for user
/// Warning: this function without "check is owner user"
fn add_user_access_standard(
    data: &IptUserAccessStandardData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableUserAccessStandard = data.into();

    diesel::insert_into(user_access_to_standard::user_access_to_standard)
        .values(&insert_data)
        .returning(user_access_to_standard::is_enabled)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed delete access for target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Удаляет доступ к стандарту для пользователя.
pub(crate) fn del_user_access_standard(
    logged_user_uuid: &Uuid,
    data: &DelUserAccessStandardData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. деактивировать доступ для указанного пользователя
    let del_access = diesel::delete(user_access_to_standard::user_access_to_standard)
        .filter(
            user_access_to_standard::standard_uuid
                .eq(&data.standard_uuid)
                .and(user_access_to_standard::user_uuid.eq(&data.user_uuid)),
        )
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
