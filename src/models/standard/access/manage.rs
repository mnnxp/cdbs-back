use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::model::{ChangeOwnerStandard, ChangeTypeAccessStandard};
use crate::models::standard::access::util::check_is_owner_with_err;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Передает право собственности на стандарт другому пользователю.
pub(crate) fn change_standard_owner_user(
    logged_user_uuid: &Uuid,
    data: &ChangeOwnerStandard,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. изменить владельца компонента
    let change_owner = diesel::update(
        standard_ref::standard_ref.filter(
            standard_ref::uuid
                .eq(&data.standard_uuid)
                .and(standard_ref::user_uuid.eq(logged_user_uuid)),
        ),
    ) // <-- на всякий пожарный :)
    .set(standard_ref::user_uuid.eq(&data.new_owner_user_uuid))
    .returning(standard_ref::user_uuid)
    .get_result::<Uuid>(conn)
    .map_err(|err| {
        debug!("Failed change owner standard: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_owner == data.new_owner_user_uuid)
}

/// Изменяет доступ по умолчанию к стандарту.
pub(crate) fn change_standard_type_access(
    logged_user_uuid: &Uuid,
    data: &ChangeTypeAccessStandard,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. изменить тип доступа компонента
    let change_access = diesel::update(
        standard_ref::standard_ref.filter(
            standard_ref::uuid
                .eq(&data.standard_uuid)
                .and(standard_ref::user_uuid.eq(logged_user_uuid)),
        ),
    ) // <-- на всякий пожарный :)
    .set(standard_ref::type_access_id.eq(&data.new_type_access_id))
    .returning(standard_ref::type_access_id)
    .get_result::<i32>(conn)
    .map_err(|err| {
        debug!("Failed change access standard: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_access == data.new_type_access_id)
}
