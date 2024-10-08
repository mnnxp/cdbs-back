use crate::errors::{ServiceResult, ServiceError};
use crate::models::search::order::Paginate;
use crate::models::standard::model::Standard;
use crate::models::standard::spec::model::{StandardSpec, StandardSpecsArg};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_standard::dsl as spec_to_standard;
use diesel::prelude::*;
// use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        arg: &StandardSpecsArg,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids = spec_to_standard::spec_to_standard
            .filter(spec_to_standard::standard_uuid.eq(arg.standard_uuid))
            .select(spec_to_standard::spec_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for standard: {:?}", err);
                ServiceError::InternalServerError
            })?;

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for standard
        SpecTranslateList::get_by_ids(
            &specs_ids,
            set_lang_id,
            &Paginate::parsing(arg.limit, arg.offset),
            conn
        ).map_err(|err| {
            debug!("Failed get specs for standard: {:?}", err);
            ServiceError::InternalServerError
        })
    }

    /// Gets all specs for standard
    pub(crate) fn for_standard(
        standard: &Standard,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids: Vec<i32> = StandardSpec::belonging_to(standard)
            .select(spec_to_standard::spec_id)
            .load::<i32>(conn)
            .expect("Error loading spec_standard");
        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }
        // get specs with translation for standard
        SpecTranslateList::get_by_ids(
            &specs_ids,
            set_lang_id,
            &Paginate::default(),
            conn
        ).map_err(|err| {
            debug!("Failed get specs for standard: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
