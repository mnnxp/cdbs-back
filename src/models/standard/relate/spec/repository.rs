use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::model::Standard;
use crate::models::standard::spec::model::{StandardSpec, StandardSpecWithTranslation, StandardSpecsArg};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_standard::dsl as spec_to_standard;
use diesel::prelude::*;
// use uuid::Uuid;

impl StandardSpecWithTranslation {
    /// Gets all specs for standard by uuid
    pub(crate) fn get_by_uuid(
        arg: &StandardSpecsArg,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardSpecWithTranslation>> {
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

        // get specs with translation for standard
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        )?;

        let mut spec_standard_with_translate: Vec<StandardSpecWithTranslation> = Vec::new();
        for x in spec_translate_list.iter() {
            spec_standard_with_translate.push(StandardSpecWithTranslation {
                spec: x.clone(),
                standard_uuid: *standard_uuid,
            })
        }

        Ok(spec_standard_with_translate)
    }

    /// Gets all specs for standard
    pub(crate) fn for_standard(
        standard: &Standard,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardSpecWithTranslation>> {
        let spec_standard: Vec<StandardSpec> = StandardSpec::belonging_to(standard)
            .load::<StandardSpec>(conn)
            .expect("Error loading spec_standard");

        // get specs for standard
        let mut spec_ids_for_standard: Vec<i32> = Vec::new();
        for spec in spec_standard.iter() {
            spec_ids_for_standard.push(spec.spec_id);
        }

        // get specs with translation for standard
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_by_ids(
            &spec_ids_for_standard,
            &100,
            &0,
            set_lang_id,
            conn
        )?;

        let mut spec_standard_with_translate: Vec<StandardSpecWithTranslation> = Vec::new();
        for x in spec_translate_list.iter() {
            spec_standard_with_translate.push(StandardSpecWithTranslation {
                spec: x.clone(),
                standard_uuid: standard.uuid,
            })
        }

        Ok(spec_standard_with_translate)
    }
}
