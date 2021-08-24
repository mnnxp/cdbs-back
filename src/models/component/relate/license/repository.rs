use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::license::model::LicenseComponent;
use crate::models::relate_ref::license::model::License;
use crate::schema::license_to_component::dsl as license_to_component;
use diesel::prelude::*;

impl License {
    /// Get list license for component
    pub fn get_by_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<License>> {
        let target_vec_id_license: Vec<i32> = LicenseComponent::belonging_to(component)
            .select(license_to_component::id_license)
            .load::<i32>(conn)?;
        License::get_license_by_vec_id(&target_vec_id_license, conn)
    }
}
