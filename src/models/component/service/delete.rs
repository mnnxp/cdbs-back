use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::model::SlimComponent;
// use crate::models::user::util::verify;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component(
    logged_user_uuid: &Uuid,
    del_component_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimComponent> {
    use crate::schema::component_ref::dsl::*;

    let delete_component = diesel::delete(component_ref
        .filter(user_uuid.eq(logged_user_uuid)
        .and(uuid.eq(del_component_uuid))))
        .returning((
            uuid,
            name,
            description,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            updated_at,
        ))
        .get_result::<SlimComponent>(conn);

    match delete_component {
        Ok(res) => Ok(res),
        Err(err) => {
            debug!("Failed delete component: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed delete component".to_string()
            ))
        }
    }
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
