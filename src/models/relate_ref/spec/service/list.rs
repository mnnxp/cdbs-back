use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    SpecTranslateList, SpecArg
};
use diesel::PgConnection;

pub(crate) fn get_specs(
    arguments: &SpecArg,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    let SpecArg {
        spec_ids,
        specs_levels,
        limit,
        offset,
    } = arguments;

    match specs_levels.is_empty() {
        true => {
            SpecTranslateList::get_by_ids(
                spec_ids,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
        false => {
            SpecTranslateList::get_by_parent_ids(
                spec_ids,
                specs_levels,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
    }
}
