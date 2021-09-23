use crate::errors::ServiceResult;
use crate::models::relate_ref::param::model::ParamTranslateList;
use diesel::prelude::*;

pub(crate) fn get_params(
    param_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    match param_id_search {
        param_id_search if param_id_search.is_empty() => find_all_params(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        param_id_search => find_param_id(
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
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;

    Ok(param_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}

fn find_param_id(
    param_id_search: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ParamTranslateList>> {
    use crate::schema::param_translate_list::dsl::*;


    Ok(param_translate_list
        .filter(param_id.eq_any(param_id_search))
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ParamTranslateList>(conn)?)
}
