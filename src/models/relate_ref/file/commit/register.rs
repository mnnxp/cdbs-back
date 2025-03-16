use super::{InsertableCommit, Commit};
use crate::errors::{ServiceResult, ServiceError};
use crate::schema::commit_ref::dsl as commit_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Commit {
    /// Creates a new comment for changes
    pub(crate) fn create_commit(
        commit_msg: &str,
        conn: &mut PgConnection
    ) -> ServiceResult<Uuid> {
        let commit_msg = InsertableCommit::new(commit_msg);
        diesel::insert_into(commit_ref::commit_ref)
            .values(&commit_msg)
            .returning(commit_ref::uuid)
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed insert commit: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}