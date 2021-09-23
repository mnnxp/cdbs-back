use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_specs(
    target_spec_ids: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match target_spec_ids {
        target_spec_ids if target_spec_ids.is_empty() => find_all_specs(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        target_spec_ids => find_spec_ids(
            target_spec_ids,
            limit,
            offset,
            set_lang_id,
            conn,
        )
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_specs(
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;

    // let target_lang_id: IdLanguage = cxt.into();

    Ok(spec_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}

fn find_spec_ids(
    target_spec_ids: Vec<i32>,
    limit: i32,
    offset: i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;


    Ok(spec_translate_list
        // .filter(lang_id.eq_any(target_lang_id))
        .filter(spec_id.eq_any(target_spec_ids))
        .filter(lang_id.eq(set_lang_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}
