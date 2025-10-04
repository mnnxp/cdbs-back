use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::extension::model::{
    Extension, InsertableExtension, IptExtensionData,
};
use crate::schema::extension_ref::dsl as extension_ref;
use diesel::prelude::*;
// use uuid::Uuid;

/// Создаёт связь расширения с программным решением.
pub(crate) fn create_extension(
    new_extension_data: &IptExtensionData,
    conn: &mut PgConnection,
) -> ServiceResult<Extension> {
    let flag_found = extension_ref::extension_ref
        .filter(extension_ref::extension.eq(&new_extension_data.extension))
        .select(extension_ref::id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check extension: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.first() {
        Some(x) => Err(get_err_msg(ErrorMessage::NameAlreadyThereX(
            "extension".to_string(),
            *x,
        ))),
        None => {
            let new_extension_data: InsertableExtension = new_extension_data.into();
            diesel::insert_into(extension_ref::extension_ref)
                .values(&new_extension_data)
                .get_result::<Extension>(conn)
                .map_err(|err| {
                    debug!("Failed insert extension: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}
