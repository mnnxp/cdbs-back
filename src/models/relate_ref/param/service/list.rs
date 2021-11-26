use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::schema::param_translate_list::dsl::*;
use diesel::prelude::*;

pub(crate) fn get_params(
    param_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match param_id_search.is_empty() {
        true => find_all_params(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        false => find_param_id(
            param_id_search,
            limit,
            offset,
            set_lang_id,
            conn,
        )
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_params(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    param_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get param: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_param_id(
    param_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    param_translate_list
        .filter(param_id.eq_any(param_id_search)
        .and(lang_id.eq(set_lang_id)))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get param: {:?}", err);
            ServiceError::InternalServerError
        })
}
