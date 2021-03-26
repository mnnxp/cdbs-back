use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
// use crate::models::user::service as user;
use crate::models::component::model::{InsertableComponent, SlimComponent, Component, ComponentData};
use crate::models::component::service as component;
// use actix_identity::{Identity, RequestIdentity};
// use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct ComponentDataQuery {
    pub name: String,
    pub comment: String,
    pub uuid_component_parent: String,
    pub id_actual_status: i32,
    pub id_component_type: i32,
    pub id_type_access: i32,
    pub is_standard: i32,
}

pub async fn register(
    new_component_data: web::Json<ComponentDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

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
