use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::param::model::{ParamComponent, ComponentParamWithTranslation};
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;

impl ComponentParamWithTranslation {
    pub fn for_component(
        component: &Component,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentParamWithTranslation>> {
        // get params component for component
        let param_component: Vec<ParamComponent> = ParamComponent::belonging_to(component)
            .load::<ParamComponent>(conn)
            .expect("Error loading param_component");

        // get params for component
        let mut id_params_component: Vec<i32> = Vec::new();
        for param in param_component.iter() {
            id_params_component.push(param.id_param);
        }

        // get params with translation for component
        let param_translate_list: Vec<ParamTranslateList> = ParamTranslateList::get_param_by_vec_id(&id_params_component, set_id_lang, conn)?;

        let mut param_component_with_translate: Vec<ComponentParamWithTranslation> = Vec::new();
        for x in param_component.iter() {
            for y in param_translate_list.iter() {
                if x.id_param == y.id_param {
                    let res: ComponentParamWithTranslation = (x.to_owned(),y.clone()).into();
                    param_component_with_translate.push(res)
                }
            }
        }

        Ok(param_component_with_translate)
    }
}
