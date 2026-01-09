use crate::errors::ServiceResult;
use crate::models::supplier_service::relate::service_status::model::ServiceStatusTranslateList;
use diesel::PgConnection;

/// Returns the list of available states (statuses) for services.
/// Filtering by status identifiers is available.
pub(crate) fn get_service_statuses(
    filter: &[i32],
    set_lang_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ServiceStatusTranslateList>> {
    ServiceStatusTranslateList::get_by_ids(filter, set_lang_id, conn)
}
