use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::license::model::License;
use crate::schema::license_ref::dsl as license_ref;
use diesel::prelude::*;

impl License {
    pub(crate) fn get_license_by_ids(
        target_vec_license_id: &[i32],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<License>> {
        license_ref::license_ref
            .filter(license_ref::id.eq_any(target_vec_license_id))
            .load::<License>(conn)
            .map_err(|err| {
                debug!("Failed get license: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
