// use crate::database::{db_connection, Pool};
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::param::model::{
    ParamToModel,
    ParamToModelData,
    InsertableParamToComponent
};
// use actix_web::web;
use diesel::prelude::*;


// pub(crate) fn add_to_component(
//     new_param_data: ParamToModelData,
//     pool: web::Data<Pool>
// ) -> ServiceResult<ParamToModel> {
//     let conn = &db_connection(&pool)?;
//     create_param_component(new_param_data, conn)
// }

pub(crate) fn create_param_component(
    new_param_data: ParamToModelData,
    conn: &PgConnection
) -> ServiceResult<ParamToModel> {
    use crate::schema::param_to_component::dsl::*;

    let flag_found_param = param_to_component
        .filter(uuid_component.eq(&new_param_data.uuid))
        .filter(id_param.eq(&new_param_data.id_param))
        // .filter(value.eq(&new_param_data.value))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param as i32 {
        0 => {
            let new_param_data: InsertableParamToComponent = new_param_data.into();
            let inserted_param_data: ParamToModel = diesel::insert_into(param_to_component)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already with the component.".to_string())),
    }
}
