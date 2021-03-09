use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
use crate::models::user::service as user;
use crate::models::file::model::{File, SlimFile, FileData};
use crate::models::file::service as file;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

pub async fn register(
    // user_data: web::Json<UserData>,
    file_data: web::Json<FileData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {

    file::register(file_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}
