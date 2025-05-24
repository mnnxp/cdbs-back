use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::license::model::License;
use crate::schema::license_to_component::dsl as license_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl License {
    /// Get list license for component
    pub(crate) fn get_by_component_uuid(
        component_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<License>> {
        let license_ids: Vec<i32> = license_to_component::license_to_component
            .filter(license_to_component::component_uuid.eq(component_uuid))
            .select(license_to_component::license_id)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Fail load license ids: {:?}", err);
                ServiceError::InternalServerError
            })?;
        License::get_license_by_ids(&license_ids, conn)
    }
}
