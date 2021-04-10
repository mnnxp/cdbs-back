use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::param::model::{
    InsertableParam,
    ParamData,
    Param
};
use actix_web::web;
use diesel::prelude::*;
// use uuid::Uuid;


pub(crate) fn register(
    new_param_data: ParamData,
    pool: web::Data<Pool>
) -> ServiceResult<Param> {
    let conn = &db_connection(&pool)?;
    create_param(new_param_data, conn)
}

pub(crate) fn create_param(
    new_param_data: ParamData,
    conn: &PgConnection
) -> ServiceResult<Param> {
    use crate::schema::param_ref::dsl::*;
    // use crate::schema::param_to_component::dsl::uuid as uuid_component;
    // use crate::schema::param_to_modification::dsl::uuid as uuid_modification;
    // use diesel::dsl::count;

    let flag_found_param = param_ref
        .filter(paramname.eq(&new_param_data.paramname))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param {
        0 => {
            let new_param_data: InsertableParam = new_param_data.into();
            let inserted_param_data: Param = diesel::insert_into(param_ref)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already there.".to_string())),
    }
}
