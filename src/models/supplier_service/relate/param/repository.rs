use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    supplier_service::param::model::ServiceParamWithTranslation,
    relate_ref::param::model::ParamTranslateList,
    search::order::{Paginate, Sort},
};
use diesel::{sql_types, prelude::*};
use uuid::Uuid;

#[derive(QueryableByName)]
struct ServiceParamAndTranslate {
    /// UUID of the service
    #[diesel(sql_type = sql_types::Uuid)]
    service_uuid: Uuid,
    /// Value of the service parameter
    #[diesel(sql_type = sql_types::Text)]
    value: String,
    /// Parameter identifier
    #[diesel(sql_type = sql_types::Integer)]
    param_id: i32,
    /// Name localization language identifier
    #[diesel(sql_type = sql_types::Integer)]
    lang_id: i32,
    /// Localized name of the parameter
    #[diesel(sql_type = sql_types::Text)]
    paramname: String,
}

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
        SELECT pt.service_uuid, pt.value, ptl.param_id, ptl.lang_id, ptl.paramname
        FROM param_to_service AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.service_uuid = '{object_uuid}' AND ptl.lang_id IN ({lang}, 1)
        {sort}
        {paginate}",
            object_uuid = service_uuid,
            lang = set_lang_id,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL service param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ServiceParamAndTranslate>(conn)
            .map_err(|err| {
                debug!("Failed get service params: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ServiceParamWithTranslation> = Vec::new();
        let mut skip = Vec::new();
        for cpt in pre_result {
            if skip.iter().any(|p_id| p_id == &cpt.param_id) {
                // this parameter has already been added
                continue;
            }
            let param = match &cpt.lang_id == set_lang_id {
                true => ParamTranslateList {
                    param_id: cpt.param_id,
                    lang_id: cpt.lang_id,
                    paramname: cpt.paramname,
                },
                false => {
                    skip.push(cpt.param_id);
                    // parameter name request with or without translation
                    ParamTranslateList::get_by_id(&cpt.param_id, set_lang_id, conn)?
                },
            };
            result.push(ServiceParamWithTranslation {
                service_uuid: cpt.service_uuid,
                param,
                value: cpt.value,
            });
        }
        Ok(result)
    }
}
