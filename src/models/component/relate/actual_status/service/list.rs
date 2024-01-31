use crate::errors::ServiceResult;
use crate::models::component::relate::actual_status::model::ActualStatusTranslateList;
use diesel::PgConnection;

/// Возвращает список доступных состояний (статусов) для компонентов.
/// Доступна фильтрация по идентификаторам статусов.
pub(crate) fn get_actual_statuses(
    filter: &[i32],
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ActualStatusTranslateList>> {
    ActualStatusTranslateList::get_by_ids(filter, set_lang_id, conn)
}
