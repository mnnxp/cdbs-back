use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::model::Component;
use crate::models::component::license::model::ComponentLicense;
use crate::models::relate_ref::license::model::License;
use crate::schema::license_to_component::dsl as license_to_component;
use diesel::prelude::*;

impl License {
    /// Get list license for component
    pub(crate) fn get_by_component(
        component: &Component,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<License>> {
        let license_ids: Vec<i32> = ComponentLicense::belonging_to(component)
            .select(license_to_component::license_id)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Fail load license ids: {:?}", err);
                ServiceError::InternalServerError
            })?;
        License::get_license_by_ids(&license_ids, conn)
    }
}
