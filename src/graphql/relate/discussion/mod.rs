pub mod discussion_model;
use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::discussion::service::update::edit_discussion_comment;
use crate::models::search::model::ExtraOptions;
use crate::models::relate_ref::discussion::{
    service::list::{get_discussions, get_discussion_comment_list},
    service::register::create_discussion_comment,
};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::graphql::relate::attributes::IptPaginate;
use super::attributes::IptSort;
use super::discussion::discussion_model::{
    DiscussionInfo, DiscussionCommentData, IptEditCommentData,
    IptDiscussionCommentData, IptObjectDiscussionsArg, IptDiscussionCommentsArg
};

#[derive(Default)]
pub struct DiscussionQuery;
#[derive(Default)]
pub struct DiscussionMutation;

#[Object]
impl DiscussionQuery {
    /// Retrieves a list of discussions associated with the specified object,
    /// applying optional filtering, sorting, and pagination.
    ///
    /// # Arguments
    ///
    /// * `args`: Input arguments for discussion retrieval, including:
    ///     + `objectUuid`: The UUID of the object associated with the discussions.
    ///     + `toObject`: The target object type (e.g., company, component, service).
    ///     + `filterByUuids`: **Optional UUID Filter**: If provided, **only discussions with the specified UUIDs are selected**. Otherwise, all discussions associated with the object are considered.
    /// * `sort`: **Optional Sorting Criteria**:
    ///     + **Available Sort Fields:**
    ///         - `title`: Sort discussions by title.
    ///         - `createdAt`: Sort discussions by creation date (default).
    ///         - `lastActivityAt`: Sort discussions by last update date.
    ///     + **Sort Order:**
    ///         - `asc` (ascending) or `desc` (descending)
    /// * `paginate`: Optional pagination settings for the discussion list.
    ///
    /// # Returns
    ///
    /// A vector of `DiscussionInfo` structs containing information about the retrieved discussions,
    /// wrapped in a `ServiceResult`.
    ///
    /// # Errors
    ///
    /// Returns a `ServiceError` if any of the following occur:
    /// * Authorization check fails.
    /// * Database connection or query execution fails.
    async fn discussions(
        &self,
        cxt: &Context<'_>,
        args: IptObjectDiscussionsArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DiscussionInfo>> {
        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_discussions(&args.into_args(sort, paginate), &options, conn)
    }

    /// Retrieves a list of discussion comments filtered by UUIDs, with optional sorting and pagination.
    ///
    /// # Arguments
    ///
    /// * `args`: Input arguments for comment retrieval, including:
    ///     + `discussionUuid`: The UUID of the discussion to which the comments belong.
    ///     + `filterByParentUuid`: The optional UUID of the parent comment (for replies).
    ///     + `filterByUuids`: UUIDs of specific comments to query (if not provided, all comments in the discussion are considered).
    /// * `sort`: **Optional Sorting Criteria**:
    ///     + **Available Sort Fields:**
    ///         - `author`: Sort comments by author.
    ///         - `parentCommentUuid`: Sort comments by parent comment.
    ///         - `createdAt`: Sort comments by creation date (default).
    ///         - `updatedAt`: Sort comments by last update date.
    ///     + **Sort Order:**
    ///         - `asc` (ascending) or `desc` (descending)
    /// * `paginate`: Optional pagination settings for the comment list.
    ///
    /// # Returns
    ///
    /// A vector of `DiscussionCommentData` structs containing information about the retrieved comments,
    /// wrapped in a `ServiceResult`.
    ///
    /// # Errors
    ///
    /// Returns a `ServiceError` if any of the following occur:
    /// * Authorization check fails.
    /// * Database connection or query execution fails.
    async fn discussion_comments(
        &self,
        cxt: &Context<'_>,
        args: IptDiscussionCommentsArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DiscussionCommentData>> {
        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_discussion_comment_list(&args.into_args(sort, paginate), &options, conn)
    }
}

#[Object]
impl DiscussionMutation {
    /// Registers a new discussion comment and returns its UUID.
    ///
    /// **Functionality:**
    /// * Creates a new discussion comment based on the provided input data.
    ///
    /// # Arguments
    ///
    /// * `args`: Input data for the new discussion comment (e.g., discussion UUID, parent comment UUID, message content).
    ///
    /// # Returns
    ///
    /// The UUID of the newly registered discussion comment, wrapped in a `ServiceResult`.
    ///
    /// # Errors
    ///
    /// Returns a `ServiceError` if any of the following occur:
    /// * Authorization check fails.
    /// * Database connection or query execution fails.
    async fn register_discussion_comment(
        &self,
        cxt: &Context<'_>,
        args: IptDiscussionCommentData,
    ) -> ServiceResult<Uuid> {
        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        debug!("Register discussion comment IptDiscussionCommentData: {:?}", args);
        create_discussion_comment(&logged_user_uuid, &args, conn)
    }

    /// Updates the content of a discussion comment and returns a boolean indicating whether the message content has been changed.
    ///
    /// **Functionality:**
    /// * Updates the discussion comment's message content based on the provided input data.
    ///
    /// # Arguments
    ///
    /// * `data`: Input data for editing the comment (e.g., comment UUID, new message content).
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the message content has been changed, wrapped in a `ServiceResult`.
    ///
    /// # Errors
    ///
    /// Returns a `ServiceError` if any of the following occur:
    /// * Authorization check fails.
    /// * Database connection or query execution fails.
    /// * The comment's message content remains unchanged after the update attempt.
    async fn edit_comment(
        &self,
        cxt: &Context<'_>,
        data: IptEditCommentData,
    ) -> ServiceResult<bool> {
        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        edit_discussion_comment(&logged_user_uuid, &data, conn)
    }
}
