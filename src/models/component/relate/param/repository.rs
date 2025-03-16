use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    component::param::model::ComponentParamWithTranslation,
    relate_ref::param::model::ParamTranslateList,
    search::order::{Paginate, Sort},
};
use diesel::{sql_types, prelude::*};
use uuid::Uuid;

#[derive(QueryableByName)]
struct ComponentParamAndTranslate {
    /// UUID of the component
    #[diesel(sql_type = sql_types::Uuid)]
    component_uuid: Uuid,
    /// Value of the component parameter
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
        SELECT pt.component_uuid, pt.value, ptl.param_id, ptl.lang_id, ptl.paramname
        FROM param_to_component AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.component_uuid = '{object_uuid}' AND ptl.lang_id IN ({lang}, 1)
        {sort}
        {paginate}",
            object_uuid = component_uuid,
            lang = set_lang_id,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL component param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ComponentParamAndTranslate>(conn)
            .map_err(|err| {
                debug!("Failed get component params: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ComponentParamWithTranslation> = Vec::new();
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
            result.push(ComponentParamWithTranslation {
                component_uuid: cpt.component_uuid,
                param,
                value: cpt.value,
            });
        }
        Ok(result)
    }
}
