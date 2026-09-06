pub(crate) mod register;
pub(crate) mod repository;

use crate::schema::*;
use uuid::Uuid;

const MAX_COMMIT_MSG_LEN: usize = 500;

/// The commit message is unique to a particular change
#[derive(Identifiable, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = commit_ref)]
pub(crate) struct Commit {
    /// Identifier for message
    pub(crate) uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = commit_ref)]
pub(crate) struct InsertableCommit {
    pub(crate) uuid: Uuid,
    pub(crate) commit_msg: String,
}

impl InsertableCommit {
    fn new(commit_msg: &str) -> Self {
        // field size in the base 500 characters
        let message = if commit_msg.chars().count() > MAX_COMMIT_MSG_LEN {
            let truncated: String = commit_msg.chars().take(MAX_COMMIT_MSG_LEN - 3).collect();
            format!("{}...", truncated)
        } else {
            commit_msg.to_string()
        };
        Self {
            uuid: Uuid::new_v4(),
            commit_msg: message,
        }
    }
}
