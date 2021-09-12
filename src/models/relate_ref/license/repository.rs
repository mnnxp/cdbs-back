use crate::errors::ServiceResult;
use crate::models::relate_ref::license::model::License;
use crate::schema::license_ref::dsl as license_ref;
use diesel::prelude::*;

impl License {
    pub fn get_license_by_id(
        target_license_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<License> {
        Ok(license_ref::license_ref
            .filter(license_ref::id.eq(target_license_id))
            .first::<License>(conn)?)
    }

    pub fn get_license_by_vec_id(
        target_vec_license_id: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<License>> {
        Ok(license_ref::license_ref
            .filter(license_ref::id.eq_any(target_vec_license_id))
            .load::<License>(conn)?)
    }
}
