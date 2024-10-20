use crate::errors::{ServiceResult, ServiceError};
use crate::models::{
    component::component_modification::param::model::ModificationParamWithTranslation,
    relate_ref::param::model::ParamTranslateList,
    search::order::{Paginate, Sort},
};
use diesel::{sql_types, prelude::*};
use uuid::Uuid;

#[derive(QueryableByName)]
struct ModificationParamAndTranslate {
    /// UUID of the component modification
    #[diesel(sql_type = sql_types::Uuid)]
    modification_uuid: Uuid,
    /// Value of the component modification parameter
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
        SELECT pt.modification_uuid, pt.value, ptl.param_id, ptl.lang_id, ptl.paramname
        FROM param_to_modification AS pt
        INNER JOIN param_translate_list AS ptl ON pt.param_id = ptl.param_id
        WHERE pt.modification_uuid = '{object_uuid}' AND ptl.lang_id IN ({lang}, 1)
        {sort}
        {paginate}",
            object_uuid = modification_uuid,
            lang = set_lang_id,
            sort = sort.get_complete(),
            paginate = paginate.get_complete(),
        );
        debug!("SQL modification param query: {}", query);
        let pre_result = diesel::sql_query(query)
            .load::<ModificationParamAndTranslate>(conn)
            .map_err(|err| {
                debug!("Failed get modification params: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ModificationParamWithTranslation> = Vec::new();
        let mut skip = Vec::new();
        for mpt in pre_result {
            if skip.iter().any(|p_id| p_id == &mpt.param_id) {
                // this parameter has already been added
                continue;
            }
            let param = match &mpt.lang_id == set_lang_id {
                true => ParamTranslateList {
                    param_id: mpt.param_id,
                    lang_id: mpt.lang_id,
                    paramname: mpt.paramname,
                },
                false => {
                    skip.push(mpt.param_id);
                    // parameter name request with or without translation
                    ParamTranslateList::get_by_id(&mpt.param_id, set_lang_id, conn)?
                },
            };
            result.push(ModificationParamWithTranslation {
                modification_uuid: mpt.modification_uuid,
                param,
                value: mpt.value,
            });
        }
        Ok(result)
    }
}
