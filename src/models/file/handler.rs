use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{
    LoggedUser
    // SlimUser,
    // UserData
};
// use crate::models::user::service as user;
use crate::models::file::model::{
    // File,
    // SlimFile,
    FileData
};
use crate::models::file::service as file;
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
pub struct RegisterFileDataQuery {
    pub uuid_file: String,
    pub hash: String,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

pub async fn register(
    new_user_data: web::Json<RegisterFileDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;
    // let default_hash = Vec::from("0".as_bytes());

    let file_data = FileData {
        path_file: (new_user_data.path_file.to_owned()),
        filesize: (new_user_data.filesize),
        id_ext: (new_user_data.id_ext),
        filename: (new_user_data.filename.to_owned()),
        uuid_user_create: (user_uuid),
        hash: (new_user_data.hash.to_owned()),
        uuid_file: (Uuid::parse_str(&new_user_data.uuid_file)?),
    };

    file::register(file_data, pool).map(|res| HttpResponse::Ok().json(&res))
}
