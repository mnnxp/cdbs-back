use crate::schema::*;
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Insertable)]
#[diesel(table_name = service_history_list)]
pub(crate) struct InsertableServiceHistoryList {
    pub(crate) service_uuid: Uuid,
    pub(crate) type_of_change_id: i32,
    pub(crate) user_uuid: Uuid,
    pub(crate) old_data: String,
    pub(crate) changed_at: NaiveDateTime,
}

impl InsertableServiceHistoryList {
    pub(crate) fn new_history_row(service_uuid: &Uuid, user_uuid: &Uuid, old_data: String) -> Self {
        Self {
            service_uuid: *service_uuid,
            type_of_change_id: 1,
            user_uuid: *user_uuid,
            old_data,
            changed_at: Local::now().naive_local(),
        }
    }
}
