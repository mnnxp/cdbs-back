use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::model::LoggedUser;
use crate::models::param::model::{
    ParamData,
    ParamToModelData
};
use crate::models::param::service as param;
use actix_web::{
    web,
    HttpResponse
};
use uuid::Uuid;

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


// ParamToModel

#[derive(Debug, Deserialize)]
pub struct NewParamToModelDataQuery {
    uuid: String,
    id_param: i32,
    value: String,
}

pub async fn add_to_component(
    new_param_to_model: web::Json<NewParamToModelDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    crate::models::user::hash_authorized(&logged_user)?;

    let new_param_to_model_data = ParamToModelData {
        uuid: (Uuid::parse_str(&new_param_to_model.uuid)?),
        id_param: (new_param_to_model.id_param),
        value: (new_param_to_model.value.to_owned()),
    };

    param::add_to_component(new_param_to_model_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn add_to_modification(
    new_param_to_model: web::Json<NewParamToModelDataQuery>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    crate::models::user::hash_authorized(&logged_user)?;

    let new_param_to_model_data = ParamToModelData {
        uuid: (Uuid::parse_str(&new_param_to_model.uuid)?),
        id_param: (new_param_to_model.id_param),
        value: (new_param_to_model.value.to_owned()),
    };

    param::add_to_modification(new_param_to_model_data, pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
