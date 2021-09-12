use crate::errors::ServiceResult;
use crate::models::standard::model::Standard;
use crate::models::standard::spec::model::{SpecStandard, StandardSpecWithTranslation};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;

impl StandardSpecWithTranslation {
    pub fn for_standard(
        standard: &Standard,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<StandardSpecWithTranslation>> {
        let spec_standard: Vec<SpecStandard> = SpecStandard::belonging_to(standard)
            .load::<SpecStandard>(conn)
            .expect("Error loading spec_standard");

        // get specs for standard
        let mut spec_ids_for_standard: Vec<i32> = Vec::new();
        for spec in spec_standard.iter() {
            spec_ids_for_standard.push(spec.spec_id);
        }

        // get specs with translation for standard
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_spec_by_vec_id(&spec_ids_for_standard, set_lang_id, conn)?;

        let mut spec_standard_with_translate: Vec<StandardSpecWithTranslation> = Vec::new();
        for x in spec_standard.iter() {
            for y in spec_translate_list.iter() {
                if x.spec_id == y.spec_id {
                    let res: StandardSpecWithTranslation = (x.to_owned(),y.clone()).into();
                    spec_standard_with_translate.push(res)
                }
            }
        }

        Ok(spec_standard_with_translate)
    }
}
