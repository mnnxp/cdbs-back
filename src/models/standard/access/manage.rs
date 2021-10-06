use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::model::{
    ChangeOwnerStandard, ChangeTypeAccessStandard,
};
use crate::models::standard::access::util::check_is_owner;
use diesel::prelude::*;
use uuid::Uuid;

/// Change standard owner user
pub(crate) fn change_standard_owner_user(
    logged_user_uuid: &Uuid,
    data: &ChangeOwnerStandard,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::standard_ref::dsl::*;

    // 1. проверить пользователя на владение стандартом
    if check_is_owner(
        logged_user_uuid,
        &data.standard_uuid,
        conn
    ) {
        // 2. изменить владельца компонента
        let change_owner = diesel::update(standard_ref
            .filter(uuid.eq(&data.standard_uuid)
            .and(user_uuid.eq(logged_user_uuid)))) // <-- на всякий пожарный :)
            .set(user_uuid.eq(data.new_owner_user_uuid))
            .returning(user_uuid)
            .get_result::<Uuid>(conn);

        match change_owner {
            Ok(x) => {
                debug!("Change standard owner, new owner: {:?}", x);

                return Ok(true)
            },
            Err(err) => {
                debug!("Failed change owner: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed change owner".to_string()
                ))
            },
        }
    }

    Err(ServiceError::BadRequest("Access denied".to_string()))
}

/// Change standard type_access
pub(crate) fn change_standard_type_access(
    logged_user_uuid: &Uuid,
    data: &ChangeTypeAccessStandard,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::standard_ref::dsl::*;

    // 1. проверить пользователя на владение стандартом
    if check_is_owner(
        logged_user_uuid,
        &data.standard_uuid,
        conn
    ) {
        // 2. изменить тип доступа компонента
        let change_access = diesel::update(standard_ref
            .filter(uuid.eq(&data.standard_uuid)
            .and(user_uuid.eq(logged_user_uuid)))) // <-- на всякий пожарный :)
            .set(type_access_id.eq(data.new_type_access_uuid))
            .returning(type_access_id)
            .get_result::<i32>(conn);

        match change_access {
            Ok(x) => {
                debug!("Change standard access, new access: {:?}", x);

                return Ok(true)
            },
            Err(err) => {
                debug!("Failed change access: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed change access".to_string()
                ))
            },
        }
    }

    Err(ServiceError::BadRequest("Access denied".to_string()))
}
