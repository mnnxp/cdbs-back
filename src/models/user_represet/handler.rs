use crate::database::{Pool, PooledConnection};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
use crate::models::user::service as user;
use crate::models::user_represet::model::{
    UserRepreset,
    SlimUserRepreset,
    UserRepresetData
};
use crate::models::user_represet::service as user_represet;
use crate::graphql::model::Context;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use crate::schema::user_represet_ref::dsl::user_represet_ref;
use std::any::Any;
use uuid::Uuid;

pub async fn register(
    user_represet_data: web::Json<UserRepresetData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user_represet::register(user_represet_data.into_inner(), pool)
        .map(|res| HttpResponse::Ok().json(&res))
}

pub fn delete(
    logged_user: LoggedUser,
    uuid_represets_delete: &str,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user_represet::delete(logged_user, uuid_represets_delete, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
