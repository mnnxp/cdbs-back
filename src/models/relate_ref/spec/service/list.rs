use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;


pub(crate) fn get_specs(
    cxt: &Context<'_>,
    target_spec_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match target_spec_id {
        target_spec_id if target_spec_id.is_empty() => find_all_specs(cxt, limit, offset),
        target_spec_id => find_spec_ids(cxt, target_spec_id, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_specs(
    cxt: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;
    // let target_lang_id: IdLanguage = cxt.into();

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(spec_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}

fn find_spec_ids(
    cxt: &Context<'_>,
    target_spec_id: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    Ok(spec_translate_list
        // .filter(lang_id.eq_any(target_lang_id))
        .filter(spec_id.eq_any(target_spec_id))
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}
