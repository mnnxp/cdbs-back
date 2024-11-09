use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{SpecTranslateList, SpecArg};
use crate::models::search::order::Paginate;
use diesel::PgConnection;

/// Returns catalogs with translation in the `SpecTranslateList` structure.
/// The top-level (parent) section is specified in the `specs_levels` from which the list will be generated.
/// Regardless of whether you specify a top section, you can filter by section IDs.
pub(crate) fn get_specs(
    args: &SpecArg,
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match args.specs_levels.is_empty() {
        true => {
            SpecTranslateList::get_by_ids(
                &args.spec_ids,
                set_lang_id,
                paginate,
                conn
            )
        },
        false => {
            SpecTranslateList::get_by_parent_ids(
                &args.spec_ids,
                &args.specs_levels,
                set_lang_id,
                paginate,
                conn
            )
        },
    }
}
