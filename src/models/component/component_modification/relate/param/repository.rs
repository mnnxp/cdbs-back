use crate::errors::ServiceResult;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::param::model::{ParamModification, ModificationParamWithTranslation};
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;

impl ModificationParamWithTranslation {
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Vec<ModificationParamWithTranslation>>> {
        // get params with grouped by modification
        let param_component_modification: Vec<Vec<ParamModification>> = ParamModification::belonging_to(component_modification)
            .load::<ParamModification>(conn)
            .expect("Error loading param_component_modification")
            .grouped_by(component_modification);


        // parsing list of id param for component modification
        let mut id_params_component_modification: Vec<i32> = Vec::new();
        for x in param_component_modification.iter() {
            for y in x.iter() {
                id_params_component_modification.push(y.id_param);
            }
        }

        // get param with translation for component modification
        let param_translate_list: Vec<ParamTranslateList> = ParamTranslateList::get_param_by_vec_id(&id_params_component_modification, set_id_lang, conn)?;

        let mut param_component_modification_with_translate: Vec<Vec<ModificationParamWithTranslation>> = Vec::new();
        for w in param_component_modification.iter() {
            for x in w.iter() {
                let mut vec_values: Vec<ModificationParamWithTranslation> = Vec::new();
                for y in param_translate_list.iter() {
                    if x.id_param == y.id_param {
                        let res: ModificationParamWithTranslation = (x.to_owned(),y.clone()).into();
                        vec_values.push(res)
                    }
                }
                param_component_modification_with_translate.push(vec_values)
            }
        }

        Ok(param_component_modification_with_translate)
    }
}
