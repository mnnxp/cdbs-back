use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    component::component_modification::param::model::ModificationParamWithTranslation,
    relate_ref::param::model::{ParamValue, ParamTranslateList},
    search::order::{Paginate, Sort},
};
use diesel::prelude::*;
use uuid::Uuid;

impl ModificationParamWithTranslation {
    /// Get parameter translate for modification
    pub(crate) fn by_modification_uuid(
        modification_uuid: &Uuid,
        set_lang_id: &i32,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ModificationParamWithTranslation>> {
        let query = format!("
        SELECT pt.param_id, pt.value
        FROM param_to_modification AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.modification_uuid = '{object_uuid}' AND ptl.lang_id = 1
        {sort}
        {paginate};",
            object_uuid = modification_uuid,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL modification param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ParamValue>(conn)
            .map_err(|err| {
                debug!("Failed get modification params: {:?}", err);
                ServiceError::InternalServerError
            })?;
        let mut result: Vec<ModificationParamWithTranslation> = Vec::new();
        for pv in pre_result {
            result.push(ModificationParamWithTranslation {
                modification_uuid: *modification_uuid,
                param: ParamTranslateList::get_by_id(&pv.param_id, set_lang_id, conn)?,
                value: pv.value,
            });
        }
        Ok(result)
    }
}
