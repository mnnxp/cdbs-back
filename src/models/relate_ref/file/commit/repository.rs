use super::Commit;
use crate::errors::{ServiceResult, ServiceError};
use crate::schema::commit_ref::dsl as commit_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Commit {
    /// Gets a message by uuid commit
    pub(crate) fn get_message(
        commit_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<String> {
        commit_ref::commit_ref
            .filter(commit_ref::uuid.eq(commit_uuid))
            .select(commit_ref::commit_msg)
            .first::<String>(conn)
            .map_err(|err| {
                debug!("Failed get commit: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
