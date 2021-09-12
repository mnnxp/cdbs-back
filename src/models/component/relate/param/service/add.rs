use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::param::model::{
    ParamComponent,
    IptParamComponentData,
    InsertableParamComponent
};
use diesel::prelude::*;

pub(crate) fn create_param_component(
    new_param_data: IptParamComponentData,
    conn: &PgConnection
) -> ServiceResult<ParamComponent> {
    use crate::schema::param_to_component::dsl::*;
    let new_param_data: InsertableParamComponent = new_param_data.into();

    let flag_found_param = param_to_component
        .filter(component_uuid.eq(&new_param_data.component_uuid))
        .filter(param_id.eq(&new_param_data.param_id))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param as i32 {
        0 => {
            let inserted_param_data: ParamComponent = diesel::insert_into(param_to_component)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already with the component.".to_string())),
    }
}
