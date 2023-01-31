use crate::errors::{ServiceResult, ServiceError};
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
        let StandardSpecsArg {
            standard_uuid,
            limit,
            offset,
        } = arg;

        let specs_ids = spec_to_standard::spec_to_standard
            .filter(spec_to_standard::standard_uuid.eq(standard_uuid))
            .select(spec_to_standard::spec_id)
            .limit(*limit as i64)
            .offset(*offset as i64)
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
            &100,
            &0,
            set_lang_id,
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
        let spec_standard: Vec<StandardSpec> = StandardSpec::belonging_to(standard)
            .load::<StandardSpec>(conn)
            .expect("Error loading spec_standard");

        // get specs for standard
        let mut specs_ids: Vec<i32> = Vec::new();
        for spec in spec_standard.iter() {
            specs_ids.push(spec.spec_id);
        }

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for standard
        SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        ).map_err(|err| {
            debug!("Failed get specs for standard: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
