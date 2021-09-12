use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::spec::model::{SpecComponent, ComponentSpecWithTranslation};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;

impl ComponentSpecWithTranslation {
    pub fn for_component(
        component: &Component,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSpecWithTranslation>> {
        let spec_component: Vec<SpecComponent> = SpecComponent::belonging_to(component)
            .load::<SpecComponent>(conn)
            .expect("Error loading spec_component");

        // get specs for component
        let mut spec_ids_for_component: Vec<i32> = Vec::new();
        for spec in spec_component.iter() {
            spec_ids_for_component.push(spec.spec_id);
        }

        // get specs with translation for component
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_spec_by_vec_id(&spec_ids_for_component, set_lang_id, conn)?;

        let mut spec_component_with_translate: Vec<ComponentSpecWithTranslation> = Vec::new();
        for x in spec_component.iter() {
            for y in spec_translate_list.iter() {
                if x.spec_id == y.spec_id {
                    let res: ComponentSpecWithTranslation = (x.to_owned(),y.clone()).into();
                    spec_component_with_translate.push(res)
                }
            }
        }

        Ok(spec_component_with_translate)
    }
}
