use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::extension::model::{InsertableExtension, Extension, IptExtensionData};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_extension(
    new_extension_data: IptExtensionData,
    conn: &PgConnection
) -> ServiceResult<Extension> {
    use crate::schema::extension_ref::dsl::*;
    // use crate::schema::extension_to_component::dsl::uuid as uuid_component;
    // use crate::schema::extension_to_modification::dsl::uuid as uuid_modification;
    // use diesel::dsl::count;

    let flag_found_extension = extension_ref
        .filter(extension.eq(&new_extension_data.extension))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_extension START SEARCH ={:?}", flag_found_extension);

    match flag_found_extension {
        0 => {
            let new_extension_data: InsertableExtension = new_extension_data.into();
            let inserted_extension_data: Extension = diesel::insert_into(extension_ref)
                .values(&new_extension_data)
                .get_result(conn)?;
            Ok(inserted_extension_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This extension name is already there. Id: {}", flag_found_extension))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
