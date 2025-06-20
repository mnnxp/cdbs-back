use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    component::param::model::ComponentParamWithTranslation,
    relate_ref::param::model::{ParamValue, ParamTranslateList},
    search::order::{Paginate, Sort},
};
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentParamWithTranslation {
    /// Get params for component by uuid
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        set_lang_id: &i32,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ComponentParamWithTranslation>> {
        let query = format!("
        SELECT pt.param_id, pt.value
        FROM param_to_component AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.component_uuid = '{object_uuid}' AND ptl.lang_id = 1
        {sort}
        {paginate};",
            object_uuid = component_uuid,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL component param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ParamValue>(conn)
            .map_err(|err| {
                debug!("Failed get component params: {:?}", err);
                ServiceError::InternalServerError
            })?;
        let mut result: Vec<ComponentParamWithTranslation> = Vec::new();
        for pv in pre_result {
            result.push(ComponentParamWithTranslation {
                component_uuid: *component_uuid,
                param: ParamTranslateList::get_by_id(&pv.param_id, set_lang_id, conn)?,
                value: pv.value,
            });
        }
        Ok(result)
    }
}
