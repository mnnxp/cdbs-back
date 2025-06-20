use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::filter::Filter;
use crate::models::search::model::ObjectI64;
use crate::schema::discussion_comment_list::dsl as discussion_comment_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Criteria for validating comments.
/// This structure is used to store the comment identifier.
/// Use implementations of this structure to define conditions and checks.
pub(crate) struct CommentCriteria { comment_uuid: Uuid }

impl CommentCriteria {
    /// Create a new instance of `CommentCriteria`
    pub(crate) fn new(comment_uuid: &Uuid) -> Self {
        CommentCriteria { comment_uuid: *comment_uuid }
    }

    /// Generates a SQL filter string to filter discussion comments by their parent UUID.
    ///
    /// **Filter Logic:**
    /// * If a parent UUID is provided (`Some(pu)`):
    ///     + Removes the parent comment from the selection if it refers to itself.
    ///     + Returns a filter string in the format: `AND uuid != parent_comment_uuid AND parent_comment_uuid = {pu}`.
    /// * If no parent UUID is provided (`None`):
    ///     + Assumes the comment is a top-level comment (i.e., its parent is itself).
    ///     + Returns a filter string: `AND uuid = parent_comment_uuid`.
    pub(crate) fn filter_by_parent(parent_uuid: Option<Uuid>) -> String {
        match parent_uuid {
            Some(pu) => {
                // Remove the parent comment from the selection if it refers to itself
                format!(
                    "AND uuid != parent_comment_uuid {}",
                    Filter::parsing("parent_comment_uuid", &[pu]).get_complete()
                )
            },
            // No parent UUID is provided, assume the comment is a top-level comment
            None => String::from("AND uuid = parent_comment_uuid"),
        }
    }

    /// Checks if a comment with the same message content already exists
    pub(crate) fn is_comment_message_present(
        &self,
        message_content: &str,
        conn: &mut PgConnection
    ) -> ServiceResult<bool> {
        let duplicate_check = discussion_comment_list::discussion_comment_list
            .select(discussion_comment_list::uuid)
            .filter(
                discussion_comment_list::uuid.eq(&self.comment_uuid)
                    .and(discussion_comment_list::message_content.eq(message_content))
            )
            .limit(1)
            .get_results::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed to check comment for duplication: {:?}", err);
                ServiceError::InternalServerError
            })?;
        match duplicate_check.first() {
            Some(dup) => {
                debug!("Found duplicate comment: {:?}", dup);
                Err(get_err_msg(ErrorMessage::FoundDuplicateData))
            },
            None => Ok(false),
        }
    }

    /// Check the comment for duplication. If the record matches all parameters, a duplicate data error will be returned.
    pub(crate) fn is_comment_duplicate(
        &self,
        discussion_uuid: &Uuid,
        parent_comment_uuid_op: Option<Uuid>,
        author_uuid: &Uuid,
        message_content: &str,
        conn: &mut PgConnection
    ) -> ServiceResult<bool> {
        let query = format!("
        SELECT count(*)
        FROM discussion_comment_list
        WHERE discussion_uuid = '{d_uuid}' {check_self_parent} {filter_author_uuid}
        AND message_content = '{message_content}'
        LIMIT 1;",
            d_uuid = discussion_uuid,
            check_self_parent = CommentCriteria::filter_by_parent(parent_comment_uuid_op),
            filter_author_uuid = Filter::parsing("author_uuid", &[*author_uuid]).get_complete()
        );
        debug!("SQL comment query for is_comment_duplicate: {}", query);
        let duplicate_check = diesel::sql_query(query)
            .get_result::<ObjectI64>(conn)
            .map_err(|err| {
                debug!("Failed count number: {:?}", err);
                ServiceError::InternalServerError
            })
            .map(|res| res.count)?;
        match duplicate_check {
            0 => Ok(false),
            _ => {
                debug!("Failed, this comment is duplicated: {:?}, message_content: {:?}", duplicate_check, message_content);
                Err(get_err_msg(ErrorMessage::FoundDuplicateData))
            },
        }
    }

    /// Checks if the specified user is the owner of the comment.
    /// This function compares the UUID of the comment owner with the provided user UUID.
    /// If they match, the function returns `false`; otherwise, it returns an error.
    pub(crate) fn verify_comment_ownership(
        &self,
        user_uuid: &Uuid,
        conn: &mut PgConnection
    ) -> ServiceResult<bool> {
        // find comment with target user
        discussion_comment_list::discussion_comment_list
            .select(discussion_comment_list::uuid)
            .filter(discussion_comment_list::uuid.eq(&self.comment_uuid)
            .and(discussion_comment_list::author_uuid.eq(user_uuid)))
            .first::<Uuid>(conn)
            .map_err(|err| {
                debug!("Not found file: {:?}", err);
                get_err_msg(ErrorMessage::AccessDenied)
            })?;
        Ok(true)
    }
}

