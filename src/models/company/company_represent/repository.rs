use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::company_represent::model::{
    CompanyRepresent, CompanyRepresentAndRelatedData, CompanyRepresentsArg,
};
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::company_represent_ref::dsl as company_represent_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyRepresent {
    /// Gets company represent without related data by company uuid
    pub(crate) fn get_by_company_uuid(
        company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresent>> {
        // collect data for represents the company
        company_represent_ref::company_represent_ref
            .filter(company_represent_ref::company_uuid.eq(company_uuid))
            .load::<CompanyRepresent>(conn)
            .map_err(|err| {
                debug!("Failed get company represents: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets company represent without related data by represents uuids
    pub(crate) fn get_by_args(
        args: &CompanyRepresentsArg,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresent>> {
        // collect data for represents the company
        company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq_any(&args.represents_uuids))
            .limit(args.limit as i64)
            .offset(args.offset as i64)
            .load::<CompanyRepresent>(conn)
            .map_err(|err| {
                debug!("Failed get company represents: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl CompanyRepresentAndRelatedData {
    /// Add company represents related data and translation for represent data
    pub(crate) fn get_by_represent(
        represent: &CompanyRepresent,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<CompanyRepresentAndRelatedData> {
        // get regions for company represent
        let region = RegionTranslateList::get_region_by_id(
            &represent.region_id,
            set_lang_id,
            conn
        )?;

        // get represent type for company represent
        let representation_type = RepresentationTypeTranslateList::get_by_id(
            &represent.representation_type_id,
            set_lang_id,
            conn
        )?;

        Ok(CompanyRepresentAndRelatedData {
            uuid: represent.uuid,
            company_uuid: represent.company_uuid,
            region,
            representation_type,
            name: represent.name.to_string(),
            address: represent.address.to_string(),
            phone: represent.phone.to_string(),
        })
    }

    /// Gets company represents by company uuid
    /// with type and region data with translation for a given language
    pub(crate) fn get_by_company_uuid(
        company_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let company_represents = &CompanyRepresent::get_by_company_uuid(
            company_uuid,
            conn
        )?;

        CompanyRepresentAndRelatedData::get_by_represents(
            company_represents,
            set_lang_id,
            conn
        )
    }

    /// Gets company represents by represents uuids
    /// with type and region data with translation for a given language
    pub(crate) fn get_by_args(
        args: &CompanyRepresentsArg,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let company_represents = &CompanyRepresent::get_by_args(
            args,
            conn
        )?;
        debug!("company_represents: {:?}", company_represents);
        CompanyRepresentAndRelatedData::get_by_represents(
            company_represents,
            set_lang_id,
            conn
        )
    }

    /// Gets company represents by company represents
    /// with type and region data with translation for a given language
    pub(crate) fn get_by_represents(
        company_represents: &[CompanyRepresent],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        debug!("company represent: {:?}", company_represents);
        let mut company_represent_with_data: Vec<CompanyRepresentAndRelatedData> = Vec::new();
        for represent in company_represents {
            company_represent_with_data.push(CompanyRepresentAndRelatedData::get_by_represent(
                represent,
                set_lang_id,
                conn
            )?)
        }
        debug!("company represent with relate: {:?}", company_represent_with_data);
        Ok(company_represent_with_data)
    }
}
