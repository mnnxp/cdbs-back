use crate::errors::ServiceResult;
use crate::models::standard::relate::standard_status::model::StandardStatusTranslateList;
use diesel::PgConnection;

/// Gets statuses list for standard
pub(crate) fn get_standard_statuses(
    filter: &[i32],
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<StandardStatusTranslateList>> {
    StandardStatusTranslateList::get_by_ids(filter, set_lang_id, conn)
}
