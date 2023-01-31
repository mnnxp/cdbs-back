use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::param::model::{
    ModificationParam, ModificationParamWithTranslation
};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_to_modification::dsl as param_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

impl ModificationParamWithTranslation {
    /// Get parameter translate for modification
    pub(crate) fn by_modification_uuid(
        modification_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ModificationParamWithTranslation>> {
        let modification_params = param_to_modification::param_to_modification
            .filter(param_to_modification::modification_uuid.eq(modification_uuid))
            .order_by(param_to_modification::param_id.asc())
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
