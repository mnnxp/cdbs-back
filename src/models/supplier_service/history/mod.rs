pub(crate) mod model;

use crate::schema::service_history_list::dsl as service_history_list;
use model::InsertableServiceHistoryList;
use diesel::prelude::*;
use uuid::Uuid;

/// Saves event to history
pub(crate) fn save_log_service_change(
    service_uuid: &Uuid,
    user_uuid: &Uuid,
    old_data: String,
    conn: &mut PgConnection,
) -> bool {
    let history_row = InsertableServiceHistoryList::new_history_row(service_uuid, user_uuid, old_data);
    // add row with notification id and target user
    let res = diesel::insert_into(service_history_list::service_history_list)
        .values(&history_row)
        .returning(service_history_list::id)
        .get_result::<i32>(conn);
    debug!("Result insert history row: {:?}", res);
    res.is_ok()
}