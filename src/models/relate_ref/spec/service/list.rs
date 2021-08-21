use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;


pub(crate) fn get_specs(
    context: &Context<'_>,
    target_id_spec: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match target_id_spec {
        target_id_spec if target_id_spec.is_empty() => find_all_specs(context, limit, offset),
        target_id_spec => find_id_specs(context, target_id_spec, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_specs(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;
    // let target_id_lang: IdLanguage = context.into();

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(spec_translate_list
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}

fn find_id_specs(
    context: &Context<'_>,
    target_id_spec: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    Ok(spec_translate_list
        // .filter(id_lang.eq_any(target_id_lang))
        .filter(id_spec.eq_any(target_id_spec))
        .filter(id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}
