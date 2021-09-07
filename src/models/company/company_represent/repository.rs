use crate::errors::ServiceResult;
use crate::models::company::model::Company;
use crate::models::company::company_represent::model::{
    CompanyRepresent,
    CompanyRepresentAndRelatedData,
};
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use diesel::prelude::*;
use uuid::Uuid;


impl CompanyRepresent {
    /// Gets company represent without related data by Company
    pub fn get_by_company(
        company: &Company,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresent>> {
        // collect data for represents the company
        Ok(CompanyRepresent::belonging_to(company)
            .load::<CompanyRepresent>(conn)?
        )
    }

    /// Gets company represent without related data by company uuid
    pub fn get_by_company_uuid(
        company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresent>> {
        use crate::schema::company_represent_ref::dsl::*;

        // collect data for represents the company
        Ok(company_represent_ref
            .filter(uuid_company.eq(company_uuid))
            .load::<CompanyRepresent>(conn)?
        )
    }

    /// Gets company represent without related data by represents uuids
    pub fn get_by_vec_uuids(
        represents_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresent>> {
        use crate::schema::company_represent_ref::dsl::*;

        // collect data for represents the company
        Ok(company_represent_ref
            .filter(uuid.eq_any(represents_uuids))
            .load::<CompanyRepresent>(conn)?
        )
    }
}

impl CompanyRepresentAndRelatedData {
    /// Gets company represents by company uuid
    /// with type and region data with translation for a given language
    pub fn get_list_represents_by_company_uuid(
        company_uuid: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let company_represents = &CompanyRepresent::get_by_company_uuid(
            company_uuid,
            conn
        ).unwrap();

        CompanyRepresentAndRelatedData::get_related_data_for_represents(
            company_represents,
            set_id_lang,
            conn
        )
    }

    /// Gets company represents by represents uuids
    /// with type and region data with translation for a given language
    pub fn get_list_represents_by_uuids(
        represents_uuids: &[Uuid],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let company_represents = &CompanyRepresent::get_by_vec_uuids(
            represents_uuids,
            conn
        ).unwrap();

        CompanyRepresentAndRelatedData::get_related_data_for_represents(
            company_represents,
            set_id_lang,
            conn
        )
    }

    /// Gets company represents by company represents
    /// with type and region data with translation for a given language
    pub fn get_related_data_for_represents(
        company_represents: &[CompanyRepresent],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let mut represent_region_list_id: Vec<i32> = Vec::new();
        let mut represent_type_list_id: Vec<i32> = Vec::new();

        // selecting represent regions and types for gets translate data
        for represent in company_represents.iter() {
            represent_region_list_id.push(represent.id_region);
            represent_type_list_id.push(represent.id_representation_type);
        }

        // get regions for company represents
        let represent_region_list_id = RegionTranslateList::get_region_by_vec_id(
            &represent_region_list_id,
            set_id_lang,
            conn
        )?;

        // get represent type for company represents
        let represent_type_list_id = RepresentationTypeTranslateList::get_representation_type_by_vec_id(
            &represent_type_list_id,
            set_id_lang,
            conn
        )?;

        // debug!("Company represent represent_type_list_id: {:#?}", represent_type_list_id);

        let mut company_represent_with_type: Vec<CompanyRepresentAndRelatedData> = Vec::new();
        for represent in company_represents {
            let mut represent_region_data = &RegionTranslateList::default();
            let mut represent_type_data = &RepresentationTypeTranslateList::default();

            // find region with translate for target represent
            for represent_region in &represent_region_list_id {
                if represent.id_region == represent_region.id_region {
                    represent_region_data = represent_region;
                }
            }

            // find represent type with translate for target represent
            for represent_type in &represent_type_list_id {
                if represent.id_representation_type == represent_type.id_representation_type {
                    represent_type_data = represent_type;
                }
            }

            company_represent_with_type.push(CompanyRepresentAndRelatedData{
                uuid: represent.uuid.to_owned(),
                uuid_company: represent.uuid_company.to_owned(),
                region:represent_region_data.to_owned(),
                representation_type:  represent_type_data.to_owned(),
                name: represent.name.to_string(),
                address: represent.address.to_string(),
                phone: represent.phone.to_string(),
            })
        }

        Ok(company_represent_with_type)
    }
}
