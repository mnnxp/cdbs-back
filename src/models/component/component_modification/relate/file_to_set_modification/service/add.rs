use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::component_modification::file_to_set_modification::model::{
    FileToSetModification,
    IptFileToSetModificationData,
    InsertableFileToSetModification
};
use diesel::prelude::*;

pub(crate) fn add_file_to_set_modification(
    new_param_data: IptFileToSetModificationData,
    conn: &PgConnection
) -> ServiceResult<FileToSetModification> {
    use crate::schema::file_to_set_modification::dsl::*;

    let new_param_data: InsertableFileToSetModification = new_param_data.into();

    let flag_found_param = file_to_set_modification
        .filter(id_set.eq(&new_param_data.id_set))
        .filter(uuid_file.eq(&new_param_data.uuid_file))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_param START SEARCH ={:?}", flag_found_param);

    match flag_found_param as i32 {
        0 => {
            let inserted_param_data: FileToSetModification = diesel::insert_into(file_to_set_modification)
                .values(&new_param_data)
                .get_result(conn)?;
            Ok(inserted_param_data)
        },
        _ => Err(ServiceError::BadRequest("This param name is already with the modification.".to_string())),
    }
}
