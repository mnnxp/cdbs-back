use crate::auth::access::{invalidate_access, invalidate_user_cache};
use crate::auth::AccessEntity;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::model::{ChangeOwnerComponent, ChangeTypeAccessComponent};
use crate::models::component::access::util::check_is_owner_with_err;
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;

/// Передает право собственности на компонент другому пользователю.
pub(crate) fn change_component_owner_user(
    logged_user_uuid: &Uuid,
    data: &ChangeOwnerComponent,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.component_uuid, conn)?;

    invalidate_access(&data.new_owner_user_uuid, AccessEntity::Component, &data.component_uuid);
    invalidate_user_cache(logged_user_uuid);

    // 2. изменить владельца компонента и обовление даты изменения компонента
    let change_owner = diesel::update(
        component_ref.filter(
            uuid.eq(&data.component_uuid)
                .and(user_uuid.eq(logged_user_uuid)),
        ),
    ) // <-- на всякий пожарный :)
    .set((
        user_uuid.eq(data.new_owner_user_uuid),
        updated_at.eq(Local::now().naive_local()),
    ))
    .returning(user_uuid)
    .get_result::<Uuid>(conn)
    .map_err(|err| {
        debug!("Failed change owner: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_owner == data.new_owner_user_uuid)
}

/// Изменяет доступ по умолчанию к компоненту.
pub(crate) fn change_component_type_access(
    logged_user_uuid: &Uuid,
    data: &ChangeTypeAccessComponent,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(logged_user_uuid, &data.component_uuid, conn)?;

    // 2. изменить тип доступа компонента и обовление даты изменения компонента
    let change_access = diesel::update(
        component_ref.filter(
            uuid.eq(&data.component_uuid)
                .and(user_uuid.eq(logged_user_uuid)),
        ),
    ) // <-- на всякий пожарный :)
    .set((
        type_access_id.eq(&data.new_type_access_id),
        updated_at.eq(Local::now().naive_local()),
    ))
    .returning(type_access_id)
    .get_result::<i32>(conn)
    .map_err(|err| {
        debug!("Failed change access: {:?}", err);
        ServiceError::InternalServerError
    })?;

    Ok(change_access == data.new_type_access_id)
}
