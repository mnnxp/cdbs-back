use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use crate::schema::spec_to_standard::dsl as spec_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        set_lang_id: &i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids = spec_to_standard::spec_to_standard
            .filter(spec_to_standard::standard_uuid.eq(standard_uuid))
            .select(spec_to_standard::spec_id)
            .order_by(spec_to_standard::spec_id.asc())
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for standard: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if specs_ids.is_empty() {
            return Ok(Vec::new()); // not found specs
        }
        // get specs with translation for standard
        SpecTranslateList::get_by_ids(&specs_ids, set_lang_id, paginate, conn).map_err(|err| {
            debug!("Failed get specs for standard: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
