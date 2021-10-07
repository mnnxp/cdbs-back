use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceError;
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
    use crate::models::user::service::login;
    use crate::models::user::access::token::{generate, decode, write_token};

    let conn: &PooledConnection = &db_connection(&pool)?;

    login(
        &auth_data.user.username,
        &auth_data.user.password,
        conn
    ).and_then(|res| {
        serde_json::to_string(&res).map_err(|_| ServiceError::InternalServerError)?;
        // debug!("user_string={}", user_string);

        // generate new token for user
        let new_token = generate(&res)?;

        match new_token.bearer {
            None => Err(ServiceError::InternalServerError),
            Some(ref token) => {
                // decrypt new token
                let new_data = decode(token.as_str())?;

                // insert data new token into the table
                write_token(token, new_data, conn)?;

                // send a token on succeed authorized
                Ok(HttpResponse::Ok().json(new_token))
            }
        }
    })
}
