use crate::database::Pool;
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::LoggedUser;
use crate::models::user_represet::model::UserRepresetData;
use crate::models::user_represet::service as user_represet;
use actix_web::{web,HttpResponse};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRepresetQuery {
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

/// Register representative for user
pub async fn register(
    new_user_represet_data: web::Json<RegisterRepresetQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    crate::models::user::has_supplier(&logged_user, 1)?;
    // crate::models::user::verify_uuid_user(&logged_user, data.uuid_user)?;

    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    // debug!("user_uuid = {}", &user_uuid);
    let user_represet_data = UserRepresetData {
        phone: (new_user_represet_data.phone.to_owned()),
        address: (new_user_represet_data.address.to_owned()),
        name: (new_user_represet_data.name.to_owned()),
        id_representation_type: (new_user_represet_data.id_representation_type),
        id_region: (new_user_represet_data.id_region),
        uuid_user: (user_uuid),
    };

    // debug!("user_represet_data = {}", &user_represet_data.uuid);
    user_represet::register(user_represet_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Deserialize)]
pub struct DeleteRepresetQuery {
    pub uuid_represet: String,
}

/// Delete representative user by uuid
pub async fn delete(
    represet_delete_data: web::Path<DeleteRepresetQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user_uuid = logged_user.0.as_ref().unwrap().uuid;

    let uuid_represet_delete = Uuid::parse_str(&represet_delete_data.uuid_represet).unwrap_or_else(|_| Uuid::nil());

    match logged_user.0 {
        None => ServiceResult::Err(ServiceError::Unauthorized),
        Some(..) if (user_uuid != Uuid::nil()) && (uuid_represet_delete != Uuid::nil()) =>
            user_represet::delete(user_uuid, uuid_represet_delete, pool)
                .map(|res| HttpResponse::Ok().json(&res)),
        _ => ServiceResult::Err(ServiceError::BadRequest("Invalid UUID".to_string())),
    }
}
