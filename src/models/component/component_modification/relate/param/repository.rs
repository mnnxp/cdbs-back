use crate::errors::ServiceResult;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::param::model::{ParamModification, ModificationParamWithTranslation};
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;

impl ModificationParamWithTranslation {
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Vec<ModificationParamWithTranslation>>> {
        // get params with grouped by modification
        let component_modification_param: Vec<Vec<ParamModification>> = ParamModification::belonging_to(component_modification)
            .load::<ParamModification>(conn)
            .expect("Error loading component_modification_param")
            .grouped_by(component_modification);


        // parsing list of id param for component modification
        let mut param_ids_component_modification: Vec<i32> = Vec::new();
        for x in component_modification_param.iter() {
            for y in x.iter() {
                param_ids_component_modification.push(y.param_id);
            }
        }

        // get param with translation for component modification
        let param_translate_list: Vec<ParamTranslateList> = ParamTranslateList::get_param_by_vec_id(&param_ids_component_modification, set_lang_id, conn)?;

        let mut component_modification_param_with_translate: Vec<Vec<ModificationParamWithTranslation>> = Vec::new();
        for w in component_modification_param.iter() {
            for x in w.iter() {
                let mut vec_values: Vec<ModificationParamWithTranslation> = Vec::new();
                for y in param_translate_list.iter() {
                    if x.param_id == y.param_id {
                        let res: ModificationParamWithTranslation = (x.to_owned(),y.clone()).into();
                        vec_values.push(res)
                    }
                }
                component_modification_param_with_translate.push(vec_values)
            }
        }

        Ok(component_modification_param_with_translate)
    }
}
