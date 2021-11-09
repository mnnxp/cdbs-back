use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    Spec, SpecTranslateList,
};
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_specs(
    target_specs_ids: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match target_specs_ids {
        target_specs_ids if target_specs_ids.is_empty() => get_all_specs(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        target_specs_ids => get_specs_by_ids(
            target_specs_ids,
            limit,
            offset,
            set_lang_id,
            conn,
        )
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn get_all_specs(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;

    // let target_lang_id: IdLanguage = cxt.into();

    Ok(spec_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<SpecTranslateList>(conn)?)
}

pub(crate) fn get_specs_by_ids(
    target_specs_ids: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_translate_list::dsl::*;

    Ok(spec_translate_list
        .filter(spec_id.eq_any(target_specs_ids)
        .and(lang_id.eq(set_lang_id)))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<SpecTranslateList>(conn)?)
}

/// Gets spec data by id
pub(crate) fn get_spec_by_id(
    target_spec_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Spec> {
    use crate::schema::spec_ref::dsl::*;

    Ok(spec_ref
        .filter(id.eq(target_spec_id))
        .first::<Spec>(conn)?)
}
