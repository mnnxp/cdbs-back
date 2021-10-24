use crate::errors::ServiceResult;
use crate::models::relate_ref::extension::model::Extension;
use crate::schema::extension_ref::dsl as extension_ref;
use diesel::prelude::*;

impl Extension {
    /// Get extension data for id
    pub fn get_by_id(
        target_extension_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Extension> {
        Ok(extension_ref::extension_ref
            .filter(extension_ref::id.eq(target_extension_id))
            .first::<Extension>(conn)?)
    }

    /// Get extension data for list id
    pub fn get_by_ids(
        target_extensions_ids: &[i32],
        limit: &i32,
        offset: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Extension>> {
        Ok(extension_ref::extension_ref
            .filter(extension_ref::id.eq_any(target_extensions_ids))
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<Extension>(conn)?)
    }

    /// Get program id for target extension id
    pub fn get_program_id(
        target_extension_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(extension_ref::extension_ref
            .filter(extension_ref::id.eq(target_extension_id))
            .select(extension_ref::program_id)
            .first::<i32>(conn)?)
    }
}
