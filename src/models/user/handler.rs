use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceError;
// use crate::models::user::model::{LoggedUser, SlimUser};
use crate::models::user::service as user;
// use actix_identity::{Identity, RequestIdentity};
// use actix_identity::Identity;
// use actix_web::dev::Payload;
// use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use actix_web::{web, HttpResponse};

// impl FromRequest for LoggedUser {
//     type Error = Error;
//     type Future = futures::future::Ready<Result<Self, Self::Error>>;
//     type Config = ();
//
//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         let identity = req.get_identity();
//
//         let slim_user = if let Some(identity) = identity {
//             match serde_json::from_str::<SlimUser>(&identity) {
//                 Err(e) => return futures::future::err(e.into()),
//                 Ok(y) => Ok(Some(y)),
//             }
//         } else {
//             Ok(None)
//         };
//
//         futures::future::ready(slim_user.map(LoggedUser))
//     }
// }

#[derive(Debug, Deserialize)]
pub(super) struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginQuery {
    pub user: UserLogin,
}

pub(super) async fn login(
    auth_data: web::Json<LoginQuery>,
    // id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let conn: &PooledConnection = &db_connection(&pool)?;

    user::login(&auth_data.user.username, &auth_data.user.password, conn).and_then(|res| {
        let user_string =
            serde_json::to_string(&res).map_err(|_| ServiceError::InternalServerError)?;
        debug!("user_string={}", user_string);
        // id.remember(user_string);
        let new_token = user::token::generate(&res)?;

        println!("Token, new_token: {:?}", &new_token);

        match new_token.bearer {
            None => Err(ServiceError::InternalServerError),
            Some(ref token) => {
                // decrypt new token
                let new_data = user::token::decode(token.as_str())?;
                println!("Token, new_data: {:?}", &new_data);
                // insert data new token into the table
                let inserted_token = user::token::write_token(token, new_data, conn)?;
                println!("Inserted token: {:#?}", inserted_token);
                Ok(HttpResponse::Ok().json(new_token))
            }
        }
    })
}

// pub fn me(logged_user: LoggedUser) -> HttpResponse {
//     match logged_user.0 {
//         None => HttpResponse::Unauthorized().json(ServiceError::Unauthorized),
//         Some(user) => HttpResponse::Ok().json(user),
//     }
// }

// pub fn logout(id: Identity) -> HttpResponse {
//     id.forget();
//     HttpResponse::Ok().finish()
// }
