pub(crate) mod register;
pub(crate) mod repository;

use crate::schema::*;
use uuid::Uuid;

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
        // field size in the base 225 characters
        let message = match commit_msg.len() > 225 {
            true => format!("{:.*}...", 222, commit_msg),
            false => commit_msg.to_string(),
        };
        Self {
            uuid: Uuid::new_v4(),
            commit_msg: message,
        }
    }
}
