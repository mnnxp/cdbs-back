use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for component by uuid
    pub(crate) fn for_component_by_uuid(
        component_uuid: &Uuid,
        set_lang_id: &i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids = spec_to_component::spec_to_component
            .filter(spec_to_component::component_uuid.eq(component_uuid))
            .select(spec_to_component::spec_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for component: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if specs_ids.is_empty() {
            return Ok(Vec::new()); // not found specs
        }
        // get specs with translation for component
        SpecTranslateList::get_by_ids(&specs_ids, set_lang_id, paginate, conn).map_err(|err| {
            debug!("Failed get specs for component: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
