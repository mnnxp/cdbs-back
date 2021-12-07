use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::model::Company;
use crate::models::company::spec::model::{CompanySpec, CompanySpecsArg};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_company::dsl as spec_to_company;
use diesel::prelude::*;
// use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for company by uuid
    pub(crate) fn for_company_by_uuid(
        arg: &CompanySpecsArg,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let CompanySpecsArg {
            company_uuid,
            limit,
            offset,
        } = arg;

        let specs_ids = spec_to_company::spec_to_company
            .filter(spec_to_company::company_uuid.eq(company_uuid))
            .select(spec_to_company::spec_id)
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for company: {:?}", err);
                ServiceError::InternalServerError
            })?;

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for company
        SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        ).map_err(|err| {
            debug!("Failed get specs for company: {:?}", err);
            ServiceError::InternalServerError
        })
    }

    /// Gets all specs for company
    pub(crate) fn for_company(
        company: &Company,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let spec_company: Vec<CompanySpec> = CompanySpec::belonging_to(company)
            .load::<CompanySpec>(conn)
            .expect("Error loading spec_company");

        // get specs for company
        let mut specs_ids: Vec<i32> = Vec::new();
        for spec in spec_company.iter() {
            specs_ids.push(spec.spec_id);
        }

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for company
        SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        ).map_err(|err| {
            debug!("Failed get specs for company: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
