use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::param::model::{ComponentParam, ComponentParamWithTranslation};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_to_component::dsl as param_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentParamWithTranslation {
    /// Get params for component by uuid
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ComponentParamWithTranslation>> {
        // get params component for component
        let component_param = param_to_component::param_to_component
            .filter(param_to_component::component_uuid.eq(component_uuid))
            .load::<ComponentParam>(conn)
            .map_err(|err| {
                debug!("Failed get param_to_component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ComponentParamWithTranslation> = Vec::new();
        for x in component_param.iter() {
            let mut data = ComponentParamWithTranslation::new(x);
            data.put_param_translate(ParamTranslateList::get_by_id(&x.param_id, set_lang_id, conn)?);
            result.push(data);
        }

        Ok(result)
    }
}
