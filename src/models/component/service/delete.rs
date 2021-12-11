use crate::errors::{ServiceResult, ServiceError};
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component(
    logged_user_uuid: &Uuid,
    del_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    diesel::delete(component_ref::component_ref
        .filter(component_ref::user_uuid.eq(logged_user_uuid)
        .and(component_ref::uuid.eq(del_component_uuid))))
        .returning(component_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete component: {:?}", err);
            ServiceError::InternalServerError
        })
}

// /// Delete component
// pub(crate) fn delete_component(
//     target_user_uuid: &Uuid,
//     target_component_uuid: &Uuid,
//     conn: &PgConnection,
// ) -> ServiceResult<bool> {
//
//     // todo!(make this function)
//
//     // 1. проверить пользователя на владение компонентом
//     // 2. установить новый флаг
//     // 3. через 30 дней удалить все данные компонента
//
//     Err(ServiceError::BadRequest("Make this soon".to_string()))
// }
