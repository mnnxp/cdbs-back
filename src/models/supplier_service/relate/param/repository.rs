use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    supplier_service::param::model::ServiceParamWithTranslation,
    relate_ref::param::model::{ParamValue, ParamTranslateList},
    search::order::{Paginate, Sort},
};
use diesel::prelude::*;
use uuid::Uuid;

impl ServiceParamWithTranslation {
    /// Get params for service by uuid
    pub(crate) fn by_service_uuid(
        service_uuid: &Uuid,
        set_lang_id: &i32,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ServiceParamWithTranslation>> {
        let query = format!("
        SELECT pt.param_id, pt.value
        FROM param_to_service AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.service_uuid = '{object_uuid}' AND ptl.lang_id = 1
        {sort}
        {paginate};",
            object_uuid = service_uuid,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL service param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ParamValue>(conn)
            .map_err(|err| {
                debug!("Failed get service params: {:?}", err);
                ServiceError::InternalServerError
            })?;
        let mut result: Vec<ServiceParamWithTranslation> = Vec::new();
        for pv in pre_result {
            result.push(ServiceParamWithTranslation {
                service_uuid: *service_uuid,
                param: ParamTranslateList::get_by_id(&pv.param_id, set_lang_id, conn)?,
                value: pv.value,
            });
        }
        Ok(result)
    }
}
