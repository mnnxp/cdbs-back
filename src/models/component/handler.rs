use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
use crate::models::component::model::{ComponentData, ComponentDataQuery};
use crate::models::component::service as component;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn register(
    new_component_data: web::Json<ComponentDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    if new_component_data.is_standard != 0 {
        crate::models::user::has_supplier(&logged_user, 1)?;
    }

    let component_data = ComponentData {
        is_standard: (new_component_data.is_standard),
        commentchange: ("Not change".to_owned()),
        id_type_access: (new_component_data.id_type_access),
        is_delete: (0),
        id_component_type: (new_component_data.id_component_type),
        id_actual_status: (new_component_data.id_actual_status),
        uuid_component_parent: (Uuid::parse_str(&new_component_data.uuid_component_parent)?),
        comment: (new_component_data.comment.to_owned()),
        uuid_user: (user_uuid),
        name: (new_component_data.name.to_owned()),
    };

    component::register(component_data, pool).map(|res| HttpResponse::Ok().json(&res))
}
