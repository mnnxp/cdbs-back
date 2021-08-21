use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::param::model::{
    ParamModification,
    IptParamModificationData,
    InsertableParamModification
};
use diesel::prelude::*;

pub(crate) fn create_param_modification(
    new_param_data: IptParamModificationData,
    conn: &PgConnection
) -> ServiceResult<ParamModification> {
    use crate::schema::param_to_modification::dsl::*;
    let new_param_data: InsertableParamModification = new_param_data.into();

    let flag_found_param = param_to_modification
        .filter(uuid_modification.eq(&new_param_data.uuid_modification))
        .filter(id_param.eq(&new_param_data.id_param))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param as i32 {
        0 => {
            let inserted_param_data: ParamModification = diesel::insert_into(param_to_modification)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already with the modification.".to_string())),
    }
}
