use crate::database::{Pool, PooledConnection};
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
// use crate::models::user::service as user;
use crate::models::user_represet::model::{
    // UserRepreset,
    // SlimUserRepreset,
    UserRepresetData
};
use crate::models::user_represet::service as user_represet;
// use crate::graphql::model::Context;
// use actix_identity::{Identity, RequestIdentity};
// use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
// use diesel::prelude::*;
// use crate::schema::user_represet_ref::dsl::user_represet_ref;
// use std::any::Any;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRepresetQuery {
    pub uuid: String,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

pub async fn register(
    new_user_represet_data: web::Json<RegisterRepresetQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    // debug!("user_uuid = {}", &user_uuid);
    let user_represet_data = UserRepresetData {
        phone: (new_user_represet_data.phone.to_owned()),
        address: (new_user_represet_data.address.to_owned()),
        name: (new_user_represet_data.name.to_owned()),
        id_representation_type: (new_user_represet_data.id_representation_type),
        id_region: (new_user_represet_data.id_region),
        uuid_user: (user_uuid),
        uuid: (Uuid::parse_str(&new_user_represet_data.uuid)?),
    };

    // debug!("user_represet_data = {}", &user_represet_data.uuid);
    user_represet::register(user_represet_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Deserialize)]
pub struct DeleteRepresetQuery {
    pub uuid_represet: String,
}

pub async fn delete(
    represet_delete_data: web::Json<DeleteRepresetQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;
    // let user_uuid = Uuid::nil();

    // debug!("represet_delete_data before parsing={}", &represet_delete_data.uuid_represet);
    let uuid_represet_delete =  Uuid::parse_str(&represet_delete_data.uuid_represet).unwrap_or(Uuid::nil());
    // debug!("uuid_represet_delete after parsing={}", &uuid_represet_delete);

    // user_represet::delete(user_uuid, uuid_represet_delete, pool)
    //             .map(|res| HttpResponse::Ok().json(&res))

    match logged_user.0 {
        None => ServiceResult::Err(ServiceError::Unauthorized),
        Some(..) if (user_uuid != Uuid::nil()) && (uuid_represet_delete != Uuid::nil()) =>
            user_represet::delete(user_uuid, uuid_represet_delete, pool)
                .map(|res| HttpResponse::Ok().json(&res)),
        // _ => ServiceResult::Err(ServiceError::BadRequest("Not valid uuid.".to_string())),
        _ => ServiceResult::Err(ServiceError::BadRequest(
            format!("user_uuid: {}, uuid_represet_delete: {}", user_uuid, uuid_represet_delete)
        )),
    }

    // if logged_user.0 == None {
    //     ServiceResult::Err(ServiceError::Unauthorized)
    // }
    // else {
    //     user_represet::delete(*user_uuid, uuid_represet_delete, pool)
    //         .map(|res| HttpResponse::Ok().json(&res))
    // }
}
