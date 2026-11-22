use crate::errors::ServiceResult;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::search::order::Paginate;
use diesel::PgConnection;

/// Returns a list of available regions with a filter by IDs.
/// If a filter is not specified, then all existing ones are aggregated.
pub(crate) fn get_regions(
    region_ids: &[i32],
    set_lang_id: i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RegionTranslateList>> {
    match region_ids.is_empty() {
        true => RegionTranslateList::find_all_regions(set_lang_id, paginate, conn),
        false => RegionTranslateList::find_region_ids(region_ids, set_lang_id, paginate, conn),
    }
}
