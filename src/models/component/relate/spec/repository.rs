use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::spec::model::{ComponentSpec, ComponentSpecWithTranslation};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;

impl ComponentSpecWithTranslation {
    pub(crate) fn for_component(
        component: &Component,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSpecWithTranslation>> {
        let component_spec: Vec<ComponentSpec> = ComponentSpec::belonging_to(component)
            .load::<ComponentSpec>(conn)
            .expect("Error loading component_spec");

        // get specs for component
        let mut spec_ids_for_component: Vec<i32> = Vec::new();
        for spec in component_spec.iter() {
            spec_ids_for_component.push(spec.spec_id);
        }

        // get specs with translation for component
        let spec_translate_list: Vec<SpecTranslateList> = SpecTranslateList::get_by_ids(
            &spec_ids_for_component,
            &100,
            &0,
            set_lang_id,
            conn
        )?;

        let mut component_spec_with_translate: Vec<ComponentSpecWithTranslation> = Vec::new();
        for x in component_spec.iter() {
            for y in spec_translate_list.iter() {
                if x.spec_id == y.spec_id {
                    let res: ComponentSpecWithTranslation = (x.to_owned(),y.clone()).into();
                    component_spec_with_translate.push(res)
                }
            }
        }

        Ok(component_spec_with_translate)
    }
}
