use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::param::model::{ParamTranslateList, ParamArg};
use crate::schema::param_translate_list::dsl::*;
use diesel::prelude::*;

pub(crate) fn get_params(
    args: &ParamArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match args.param_ids.is_empty() {
        true => get_all_params(&args.limit, &args.offset, set_lang_id, conn),
        false => get_by_ids(args, set_lang_id, conn),
    }
}

fn get_all_params(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &mut PgConnection,
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

fn get_by_ids(
    args: &ParamArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    param_translate_list
        .filter(param_id.eq_any(&args.param_ids)
        .and(lang_id.eq(set_lang_id)))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<ParamTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get param: {:?}", err);
            ServiceError::InternalServerError
        })
}
