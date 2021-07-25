use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceError;
use crate::models::user::service as user;
use actix_web::{web, HttpResponse};

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
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let conn: &PooledConnection = &db_connection(&pool)?;

    user::login(&auth_data.user.username, &auth_data.user.password, conn).and_then(|res| {
        let user_string =
            serde_json::to_string(&res).map_err(|_| ServiceError::InternalServerError)?;
        debug!("user_string={}", user_string);

        // generate new token for user
        let new_token = user::token::generate(&res)?;
        // println!("Token, new_token: {:?}", &new_token);

        match new_token.bearer {
            None => Err(ServiceError::InternalServerError),
            Some(ref token) => {
                // decrypt new token
                let new_data = user::token::decode(token.as_str())?;
                // insert data new token into the table
                user::token::write_token(token, new_data, conn)?;
                // send a token on succeed authorized
                Ok(HttpResponse::Ok().json(new_token))
            }
        }
    })
}
