use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::models::search::order::Paginate;
use crate::schema::param_translate_list::dsl::*;
use diesel::prelude::*;

/// Returns a list of available parameters.
/// If no parameter filter is specified, all existing parameters are aggregated.
pub(crate) fn get_params(
    param_ids: &[i32],
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match param_ids.is_empty() {
        true => get_all_params(set_lang_id, paginate, conn),
        false => get_by_ids(param_ids, set_lang_id, paginate, conn),
    }
}

fn get_all_params(
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    param_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get param: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn get_by_ids(
    param_ids: &[i32],
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    param_translate_list
        .filter(param_id.eq_any(param_ids)
        .and(lang_id.eq(set_lang_id)))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get param: {:?}", err);
            ServiceError::InternalServerError
        })
}
