use crate::database::{db_connection, Pool, PooledConnection};
use crate::errors::ServiceError;
use actix_web::{web, HttpResponse};

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub user: UserLogin,
}

#[derive(Debug, Serialize)]
pub struct ReturnToken {
    pub bearer: String,
}

pub(super) async fn login(
    auth_data: web::Json<LoginQuery>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    use crate::models::user::access::login::login_with_pass;

    let conn: &PooledConnection = &db_connection(&pool)?;

    let new_token = login_with_pass(
        &auth_data.user.username,
        &auth_data.user.password,
        conn
    )?;

    // send a token on succeed authorized
    Ok(HttpResponse::Ok().json(
        ReturnToken{
            bearer: new_token.token
        }
    ))
}
