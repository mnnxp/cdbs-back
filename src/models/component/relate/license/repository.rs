use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::license::model::ComponentLicense;
use crate::models::relate_ref::license::model::License;
use crate::schema::license_to_component::dsl as license_to_component;
use diesel::prelude::*;

impl License {
    /// Get list license for component
    pub(crate) fn get_by_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<License>> {
        let target_vec_license_id: Vec<i32> = ComponentLicense::belonging_to(component)
            .select(license_to_component::license_id)
            .load::<i32>(conn)?;
        License::get_license_by_ids(&target_vec_license_id, conn)
    }
}
