use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::license::model::LicenseComponent;
use crate::models::relate_ref::license::model::License;
use crate::schema::license_ref::dsl as license_ref;
use crate::schema::license_to_component::dsl as license_to_component;
use diesel::prelude::*;

impl License {
    pub fn get_license_by_id(
        target_id_license: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<License> {
        Ok(license_ref::license_ref
            .filter(license_ref::id.eq(target_id_license))
            .first::<License>(conn)?)
    }

    pub fn get_license_by_vec_id(
        target_vec_id_license: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<License>> {
        Ok(license_ref::license_ref
            .filter(license_ref::id.eq_any(target_vec_id_license))
            .load::<License>(conn)?)
    }

    pub fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<License>> {
        let target_id_license: Vec<i32> = LicenseComponent::belonging_to(component)
            .select(license_to_component::id_license)
            .load::<i32>(conn)?;
        Ok(license_ref::license_ref
            .filter(license_ref::id.eq_any(target_id_license))
            .load::<License>(conn)?)
    }
}
