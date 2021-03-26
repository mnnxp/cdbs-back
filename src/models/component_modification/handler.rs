use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
use crate::models::user::service as user;
use crate::models::component_modification::model::{
    // InsertableComponentModification,
    // SlimComponentModification,
    // ComponentModification,
    ComponentModificationData
};
use crate::models::component_modification::service as modification_list;
// use actix_identity::{Identity, RequestIdentity};
// use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

pub async fn register(
    // user_data: web::Json<UserData>,
    component_modification_data: web::Json<ComponentModificationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {

    modification_list::register(component_modification_data.into_inner(), pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
