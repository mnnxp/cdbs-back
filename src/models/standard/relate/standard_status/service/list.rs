use crate::errors::ServiceResult;
use crate::models::standard::relate::standard_status::model::StandardStatusTranslateList;
use diesel::PgConnection;

/// Возвращает список доступных состояний (статусов) для стандартов.
/// Доступна фильтрация по идентификаторам статусов.
pub(crate) fn get_standard_statuses(
    filter: &[i32],
    set_lang_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<StandardStatusTranslateList>> {
    StandardStatusTranslateList::get_by_ids(filter, set_lang_id, conn)
}
