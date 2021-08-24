use crate::errors::ServiceResult;
use crate::models::relate_ref::extension::model::Extension;
use crate::schema::extension_ref::dsl as extension_ref;
use diesel::prelude::*;

impl Extension {
    /// Get extension data for id
    pub fn get_by_id(
        target_id_extension: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Extension> {
        Ok(extension_ref::extension_ref
            .filter(extension_ref::id.eq(target_id_extension))
            .first::<Extension>(conn)?)
    }

    /// Get extension data for list id
    pub fn get_by_vec_id(
        target_list_id_extension: &[i32],
        limit: i32,
        offset: i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Extension>> {
        Ok(extension_ref::extension_ref
            .filter(extension_ref::id.eq_any(target_list_id_extension))
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<Extension>(conn)?)
    }
}
