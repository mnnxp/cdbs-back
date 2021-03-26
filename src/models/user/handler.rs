use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::{LoggedUser, SlimUser, UserData};
use crate::models::user::service as user;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use uuid::Uuid;


impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();

        let slim_user = if let Some(identity) = identity {
            match serde_json::from_str::<SlimUser>(&identity) {
                Err(e) => return futures::future::err(e.into()),
                Ok(y) => Ok(Some(y)),
            }
        } else {
            Ok(None)
        };

        futures::future::ready(slim_user.map(LoggedUser))
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserDataQuery {
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: String,
    pub id_region: i32,
}

pub async fn register(
    new_user_data: web::Json<RegisterUserDataQuery>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // let is_supplier = 0;
    // let id_name_cad = new_user_data.id_name_cad.parse::<i32>().unwrap_or(1);
    // let id_type_user = new_user_data.id_type_user.parse::<i32>().unwrap_or(1);
    // let id_region = new_user_data.id_region.parse::<i32>().unwrap_or(1);
    let uuid_file_info_icon = Uuid::parse_str(&new_user_data.uuid_file_info_icon)
        .unwrap_or(Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?);

    let user_data = match new_user_data.is_supplier {
        1 if new_user_data.id_type_user != 1 => {
            UserData {
                id_region: (new_user_data.id_region),
                uuid_file_info_icon: (uuid_file_info_icon),
                site_url: (new_user_data.site_url.to_owned()),
                position: (new_user_data.position.to_owned()),
                time_zone: (new_user_data.time_zone),
                address: (new_user_data.address.to_owned()),
                comment: (new_user_data.comment.to_owned()),
                id_name_cad: (new_user_data.id_name_cad),
                phone: (new_user_data.phone.to_owned()),
                inn: (new_user_data.inn.to_owned()),
                shortname: (new_user_data.shortname.to_owned()),
                orgname: (new_user_data.orgname.to_owned()),
                is_supplier: 1,
                id_type_user: (new_user_data.id_type_user),
                password: (new_user_data.password.to_owned()),
                email: (new_user_data.email.to_owned()),
                nickname: (new_user_data.nickname.to_owned()),
                secondname: (new_user_data.secondname.to_owned()),
                lastname: (new_user_data.lastname.to_owned()),
                firstname: (new_user_data.firstname.to_owned()),
            }
        },
        _ => {
            UserData {
                id_region: (new_user_data.id_region),
                uuid_file_info_icon: (uuid_file_info_icon),
                site_url: ("none".to_owned()),
                position: ("engineer".to_owned()),
                time_zone: 3,
                address: (new_user_data.address.to_owned()),
                comment: (new_user_data.comment.to_owned()),
                id_name_cad: (new_user_data.id_name_cad),
                phone: (new_user_data.phone.to_owned()),
                inn: ("none".to_owned()),
                shortname: ("none".to_owned()),
                orgname: ("none".to_owned()),
                is_supplier: 0,
                id_type_user: (new_user_data.id_type_user),
                password: (new_user_data.password.to_owned()),
                email: (new_user_data.email.to_owned()),
                nickname: (new_user_data.nickname.to_owned()),
                secondname: (new_user_data.secondname.to_owned()),
                lastname: (new_user_data.lastname.to_owned()),
                firstname: (new_user_data.firstname.to_owned()),
            }
        },
    };

    user::register(user_data, pool).map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginQuery {
    pub nickname: String,
    pub password: String,
}

pub(super) async fn login(
    auth_data: web::Json<LoginQuery>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user::login(&auth_data.nickname, &auth_data.password, pool).and_then(|res| {
        let user_string =
            serde_json::to_string(&res).map_err(|_| ServiceError::InternalServerError)?;
        debug!("user_string={}", user_string);
        id.remember(user_string);
        Ok(HttpResponse::Ok().json(res))
    })
}

pub fn me(logged_user: LoggedUser) -> HttpResponse {
    match logged_user.0 {
        None => HttpResponse::Unauthorized().json(ServiceError::Unauthorized),
        Some(user) => HttpResponse::Ok().json(user),
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}
