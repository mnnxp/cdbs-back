use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::PgConnection;

pub(crate) fn get_specs(
    target_specs_ids: &[i32],
    target_specs_levels: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match target_specs_levels.is_empty() {
        true => {
            SpecTranslateList::get_by_ids(
                target_specs_ids,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
        false => {
            SpecTranslateList::get_by_parent_ids(
                target_specs_ids,
                target_specs_levels,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
    }
}
