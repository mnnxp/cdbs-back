use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
// use crate::models::file::model::FileData;
use crate::models::file::service as file;
use actix_web::{web, HttpResponse};
use actix_multipart::Multipart;

// use uuid::Uuid;

pub async fn register(
    // new_user_data: web::Json<RegisterFileDataQuery>,
    payload: Multipart,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let respond_slim_file = file::register(payload, user_uuid, pool).await?;

    Ok(HttpResponse::Ok().json(&respond_slim_file))
}
