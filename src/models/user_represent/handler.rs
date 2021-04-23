use crate::database::Pool;
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::LoggedUser;
use crate::models::user_represent::model::UserRepresentData;
use crate::models::user_represent::service as user_represent;
use actix_web::{web,HttpResponse};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRepresentQuery {
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

/// Register representative for user
pub async fn register(
    new_user_represent_data: web::Json<RegisterRepresentQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    crate::models::user::has_supplier(&logged_user, 1)?;
    // crate::models::user::verify_uuid_user(&logged_user, data.uuid_user)?;

    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    // debug!("user_uuid = {}", &user_uuid);
    let user_represent_data = UserRepresentData {
        phone: (new_user_represent_data.phone.to_owned()),
        address: (new_user_represent_data.address.to_owned()),
        name: (new_user_represent_data.name.to_owned()),
        id_representation_type: (new_user_represent_data.id_representation_type),
        id_region: (new_user_represent_data.id_region),
        uuid_user: (user_uuid),
    };

    // debug!("user_represent_data = {}", &user_represent_data.uuid);
    user_represent::register(user_represent_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Deserialize)]
pub struct DeleteRepresentQuery {
    pub uuid_represent: String,
}

/// Delete representative user by uuid
pub async fn delete(
    represent_delete_data: web::Path<DeleteRepresentQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let uuid_represent_delete = Uuid::parse_str(&represent_delete_data.uuid_represent).unwrap_or_else(|_| Uuid::nil());

    match logged_user.0 {
        None => ServiceResult::Err(ServiceError::Unauthorized),
        Some(..) if (user_uuid != Uuid::nil()) && (uuid_represent_delete != Uuid::nil()) =>
            user_represent::delete(user_uuid, uuid_represent_delete, pool)
                .map(|res| HttpResponse::Ok().json(&res)),
        _ => ServiceResult::Err(ServiceError::BadRequest("Invalid UUID".to_string())),
    }
}
