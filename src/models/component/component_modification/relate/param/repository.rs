use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::{
    model::ComponentModification,
    param::model::{ModificationParam, ModificationParamWithTranslation},
};
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;

impl ModificationParamWithTranslation {
    /// Get parameter translate for modification
    pub(crate) fn for_modificaiton(
        component_modification: &ComponentModification,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ModificationParamWithTranslation>> {
        let modification_params = ModificationParam::belonging_to(component_modification)
            .load::<ModificationParam>(conn)
            .map_err(|err| {
                debug!("Failed get modification params: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ModificationParamWithTranslation> = Vec::new();
        for x in modification_params.iter() {
            let mut data = ModificationParamWithTranslation::new(x);
            data.put_param_translate(ParamTranslateList::get_by_id(
                &x.param_id,
                set_lang_id,
                conn
            )?);
            result.push(data);
        }

        Ok(result)
    }
}
