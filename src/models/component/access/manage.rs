use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::model::{
    ChangeOwnerComponent, ChangeTypeAccessComponent,
};
use crate::models::component::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Change component owner user
pub(crate) fn change_component_owner_user(
    logged_user_uuid: &Uuid,
    data: &ChangeOwnerComponent,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(
        logged_user_uuid,
        &data.component_uuid,
        conn
    )?;

    // 2. изменить владельца компонента
    let change_owner = diesel::update(component_ref
        .filter(uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(logged_user_uuid)))) // <-- на всякий пожарный :)
        .set(user_uuid.eq(data.new_owner_user_uuid))
        .returning(user_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed change owner: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(change_owner == data.new_owner_user_uuid)
}

/// Change component type_access
pub(crate) fn change_component_type_access(
    logged_user_uuid: &Uuid,
    data: &ChangeTypeAccessComponent,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::component_ref::dsl::*;

    // 1. проверить пользователя на владение компонентом
    check_is_owner_with_err(
        logged_user_uuid,
        &data.component_uuid,
        conn
    )?;

    // 2. изменить тип доступа компонента
    let change_access = diesel::update(component_ref
        .filter(uuid.eq(&data.component_uuid)
        .and(user_uuid.eq(logged_user_uuid)))) // <-- на всякий пожарный :)
        .set(type_access_id.eq(&data.new_type_access_id))
        .returning(type_access_id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!("Failed change access: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(change_access == data.new_type_access_id)
}
