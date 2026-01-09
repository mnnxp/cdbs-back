use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::extension::model::Extension;
use crate::schema::extension_ref::dsl as extension_ref;
use diesel::prelude::*;

impl Extension {
    /// Get program id for target extension id
    pub(crate) fn get_program_id(
        target_extension_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        extension_ref::extension_ref
            .filter(extension_ref::id.eq(target_extension_id))
            .select(extension_ref::program_id)
            .first::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get extension: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
