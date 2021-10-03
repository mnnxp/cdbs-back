use crate::errors::ServiceResult;
use crate::models::company::model::Company;
use crate::models::company::spec::model::{CompanySpec, CompanySpecWithTranslation};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;

impl CompanySpecWithTranslation {
    pub fn for_company(
        company: &Company,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanySpecWithTranslation>> {
        let spec_company: Vec<CompanySpec> = CompanySpec::belonging_to(company)
            .load::<CompanySpec>(conn)
            .expect("Error loading spec_company");

        // get specs for company
        let mut spec_ids_for_company: Vec<i32> = Vec::new();
        for spec in spec_company.iter() {
            spec_ids_for_company.push(spec.spec_id);
        }

        // get specs with translation for company
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_spec_by_vec_id(&spec_ids_for_company, set_lang_id, conn)?;

        let mut spec_company_with_translate: Vec<CompanySpecWithTranslation> = Vec::new();
        for x in spec_company.iter() {
            for y in spec_translate_list.iter() {
                if x.spec_id == y.spec_id {
                    let res: CompanySpecWithTranslation = (x.to_owned(),y.clone()).into();
                    spec_company_with_translate.push(res)
                }
            }
        }

        Ok(spec_company_with_translate)
    }
}
