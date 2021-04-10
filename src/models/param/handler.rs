use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
use crate::models::param::model::ParamData;
use crate::models::param::service as param;
use actix_web::{
    web,
    HttpResponse
};
// use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewParamnameDataQuery {
    paramname: String,
}

// don't forget to fix it, it doesn't look very good
pub async fn register(
    new_param_name: web::Json<NewParamnameDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    crate::models::user::hash_authorized(&logged_user)?;

    let new_param_data = ParamData {  paramname: (new_param_name.paramname.to_owned()) };

    param::register(new_param_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
