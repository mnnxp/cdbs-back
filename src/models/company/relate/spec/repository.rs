use crate::errors::{ServiceResult, ServiceError};
use crate::models::search::order::Paginate;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_company::dsl as spec_to_company;
use diesel::prelude::*;
use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for company by uuid
    pub(crate) fn for_company_by_uuid(
        company_uuid: &Uuid,
        set_lang_id: &i32,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids = spec_to_company::spec_to_company
            .filter(spec_to_company::company_uuid.eq(company_uuid))
            .select(spec_to_company::spec_id)
            .order_by(spec_to_company::spec_id.asc())
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for company: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }
        // get specs with translation for company
        SpecTranslateList::get_by_ids(&specs_ids, set_lang_id, paginate, conn)
            .map_err(|err| {
                debug!("Failed get specs for company: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets all specs for company
    pub(crate) fn for_company_uuid(
        company_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let specs_ids: Vec<i32> = spec_to_company::spec_to_company
            .filter(spec_to_company::company_uuid.eq(company_uuid))
            .select(spec_to_company::spec_id)
            .load::<i32>(conn)
            .expect("Error loading spec_company");
        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }
        // get specs with translation for company
        SpecTranslateList::get_by_ids(
            &specs_ids,
            set_lang_id,
            &Paginate::default(),
            conn
        ).map_err(|err| {
            debug!("Failed get specs for company: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
