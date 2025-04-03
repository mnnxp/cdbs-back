use crate::errors::{ServiceResult, ServiceError};
use crate::models::search::order::Paginate;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_service::dsl as spec_to_service;
use diesel::prelude::*;
use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for service by uuid
    pub(crate) fn for_service_by_uuid(
        service_uuid: &Uuid,
        set_lang_id: &i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids = spec_to_service::spec_to_service
            .filter(spec_to_service::service_uuid.eq(service_uuid))
            .select(spec_to_service::spec_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for service: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }
        // get specs with translation for service
        SpecTranslateList::get_by_ids(&specs_ids, set_lang_id, paginate, conn)
            .map_err(|err| {
                debug!("Failed get specs for service: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
