use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{
    LoggedUser
    // SlimUser,
    // UserData
};
// use crate::models::user::service as user;
use crate::models::component_modification::model::{
    // InsertableComponentModification,
    // SlimComponentModification,
    // ComponentModification,
    ComponentModificationData
};
use crate::models::component_modification::service as modification_list;
// use actix_identity::{Identity, RequestIdentity};
// use actix_web::dev::Payload;
use actix_web::{
    web,
    // Error,
    // FromRequest,
    // HttpRequest,
    HttpResponse
};
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct ComponentModificationDataQuery {
    pub uuid_component: String,
    pub modification_name: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub uuid_modification_parent: String,
    pub id_actual_status: i32,
}

pub async fn register(
    new_component_modification_data: web::Json<ComponentModificationDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let _user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let component_modification_data = ComponentModificationData {
        id_actual_status: (new_component_modification_data.id_actual_status.to_owned()),
        uuid_modification_parent: (Uuid::parse_str(&new_component_modification_data.uuid_modification_parent)?),
        comment: (new_component_modification_data.comment.to_owned()),
        id_name_cad: (new_component_modification_data.id_name_cad),
        modification_name: (new_component_modification_data.modification_name.to_owned()),
        uuid_component: (Uuid::parse_str(&new_component_modification_data.uuid_component)?),
    };

    modification_list::register(component_modification_data, _user_uuid, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
