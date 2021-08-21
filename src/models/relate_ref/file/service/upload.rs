use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
// use crate::models::relate_ref::file::model::FileData;
use crate::models::relate_ref::file::service as file;
use actix_web::{web, HttpResponse};
use actix_multipart::Multipart;

use uuid::Uuid;

#[derive(Deserialize)]
pub struct ParamQuery {
    uuid: String,
}

/// Upload images for profile picture
pub async fn add_user(
    payload: Multipart,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let addiction_table: u8 = 1_u8;
    let uuid_addiction = Uuid::nil();

    let respond_slim_file = file::register(
        payload, user_uuid, addiction_table, uuid_addiction, pool
    ).await?;

    Ok(HttpResponse::Ok().json(&respond_slim_file))
}

/// Upload files for component (documents, specifications etc.)
pub async fn add_component(
    uuid_component: web::Path<ParamQuery>,
    payload: Multipart,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let addiction_table: u8 = 2_u8;
    // let uuid_addiction = Uuid::parse_str(&uuid_component)?;
    let uuid_addiction = Uuid::parse_str(uuid_component.0.uuid.as_str())?;

    let respond_slim_file = file::register(
        payload, user_uuid, addiction_table, uuid_addiction, pool
    ).await?;

    Ok(HttpResponse::Ok().json(&respond_slim_file))
}

/// Upload files for component modifications (drawings, models etc.)
pub async fn add_modification(
    uuid_modification: web::Path<ParamQuery>,
    payload: Multipart,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let addiction_table: u8 = 3_u8;
    // let uuid_addiction = Uuid::parse_str(&uuid_modification)?;
    let uuid_addiction = Uuid::parse_str(uuid_modification.0.uuid.as_str())?;

    let respond_slim_file = file::register(
        payload, user_uuid, addiction_table, uuid_addiction, pool
    ).await?;

    Ok(HttpResponse::Ok().json(&respond_slim_file))
}
