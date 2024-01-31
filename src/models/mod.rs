pub(crate) mod company;
pub(crate) mod component;
pub(crate) mod standard;
pub(crate) mod user;
pub(crate) mod relate_ref;

use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct ExtraOptions {
    pub(crate) logged_user_uuid: Uuid,
    pub(crate) set_lang_id: i32,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl ExtraOptions {
    pub(crate) fn from_ipt(
        logged_user_uuid: Uuid,
        set_lang_id: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Self {
        Self {
            logged_user_uuid,
            set_lang_id,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}