use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
use crate::models::user::service as user;
use crate::models::component::model::{InsertableComponent, SlimComponent, Component, ComponentData};
use crate::models::component::service as component;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

pub async fn register(
    // user_data: web::Json<UserData>,
    component_data: web::Json<ComponentData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {

    component::register(component_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}
