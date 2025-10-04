use async_graphql::{self, Context, Enum, InputObject, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::relate::attributes::{IptPaginate, IptSort};
use crate::models::relate_ref::discussion::model::Discussion;
use crate::models::relate_ref::discussion::model::{
    CommentQueryOptions, DiscussQueryOptions, DiscussionCommentList, DiscussionTo,
};
use crate::models::relate_ref::discussion::service::list::get_discussion_comment_list;
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::{Paginate, Sort, TableName};
use crate::models::user::model::ShowUserShort;

/// Default sorting: `createdAt`. Sorting by `title`, `isPinned`, `updatedAt` (last activity) is available.
#[derive(Debug)]
pub(crate) struct DiscussionInfo {
    /// Discussion identifier
    pub(crate) uuid: Uuid,
    /// Discussion title
    pub(crate) title: String,
    /// Flag to pinned the discussion
    pub(crate) is_pinned: bool,
    /// Recent activity in the discussion
    pub(crate) last_activity_at: NaiveDateTime,
    /// Discussion creation date
    pub(crate) created_at: NaiveDateTime,
}

#[Object]
impl DiscussionInfo {
    /// Discussion identifier
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Discussion title
    async fn title(&self) -> &String {
        &self.title
    }

    /// Flag to pinned the discussion
    async fn is_pinned(&self) -> &bool {
        &self.is_pinned
    }

    /// Recent activity in the discussion
    async fn last_activity_at(&self) -> &NaiveDateTime {
        &self.last_activity_at
    }

    /// Discussion creation date
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Returns the number of comments left on this discussion
    async fn replies_count(&self, cxt: &Context<'_>) -> ServiceResult<i64> {
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        Discussion::comments_count(&self.uuid, conn)
    }

    /// Retrieves a list of discussion comments filtered by UUIDs, with optional sorting and pagination.
    ///
    /// # Arguments
    ///
    /// * `filterByUuids`: UUIDs of specific comments to query (if not provided, all comments in the discussion are considered).
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
    async fn comments(
        &self,
        cxt: &Context<'_>,
        filter_by_uuids: Option<Vec<Uuid>>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DiscussionCommentData>> {
        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let s = sort
            .map(|s| Sort::parsing(TableName::DiscussionCommentList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::DiscussionCommentList));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_discussion_comment_list(
            &CommentQueryOptions::new(self.uuid, None, filter_by_uuids.unwrap_or_default(), s, p),
            &options,
            conn,
        )
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptDiscussionData {
    /// Discussion title
    pub(crate) title: String,
    /// Flag to pinned the discussion
    #[graphql(default = false)]
    pub(crate) is_pinned: bool,
}

/// Default sorting: `createdAt`. Sorting by `updatedAt` is available.
#[derive(Debug)]
pub(crate) struct DiscussionCommentData {
    /// Discussion identifier
    pub(crate) uuid: Uuid,
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Uuid,
    /// Parent comment identifier
    pub(crate) parent_comment_uuid: Uuid,
    /// Data about the user who created the comment
    pub(crate) author: ShowUserShort,
    /// Content of the comment message
    pub(crate) message_content: String,
    /// Сomment editing indicator
    pub(crate) is_edited: bool,
    /// Сomment hiding indicator
    pub(crate) is_hidden: bool,
    /// Comment creation date
    pub(crate) created_at: NaiveDateTime,
    /// Comment modification date
    pub(crate) updated_at: NaiveDateTime,
}

#[Object]
impl DiscussionCommentData {
    /// Discussion identifier
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Discussion thread identifier
    async fn discussion_uuid(&self) -> &Uuid {
        &self.discussion_uuid
    }

    /// Parent comment identifier
    async fn parent_comment_uuid(&self) -> &Uuid {
        &self.parent_comment_uuid
    }

    /// Data about the user who created the comment
    async fn author(&self) -> &ShowUserShort {
        &self.author
    }

    /// Content of the comment message
    async fn message_content(&self) -> &String {
        &self.message_content
    }

    /// Сomment editing indicator
    async fn is_edited(&self) -> &bool {
        &self.is_edited
    }

    /// Сomment hiding indicator
    async fn is_hidden(&self) -> &bool {
        &self.is_hidden
    }

    /// Comment creation date
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Comment modification date
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Returns the number of comments-replies left on this comment
    async fn replies_count(&self, cxt: &Context<'_>) -> ServiceResult<i64> {
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        DiscussionCommentList::replies_count(&self.uuid, conn)
    }

    /// Returns comments that were posted in reply to this commentary
    async fn replies(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DiscussionCommentData>> {
        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let s = sort
            .map(|s| Sort::parsing(TableName::DiscussionCommentList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::DiscussionCommentList));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_discussion_comment_list(
            &CommentQueryOptions::new(self.discussion_uuid, Some(self.uuid), Vec::new(), s, p),
            &options,
            conn,
        )
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptDiscussionCommentData {
    /// Object with which the comment is associated (must be specified)
    pub(crate) object_discussion: IptDiscussionArg,
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Option<Uuid>,
    /// Parent comment identifier
    pub(crate) parent_comment_uuid: Option<Uuid>,
    /// Content of the comment message
    pub(crate) message_content: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptEditCommentData {
    /// Discussion comment identifier
    pub(crate) comment_uuid: Uuid,
    /// New text for content of message
    pub(crate) updated_message: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Deserialize, Debug)]
pub(crate) enum ToObject {
    Company,
    Component,
    Service,
}

impl ToObject {
    fn discuss_to(&self, object_uuid: Uuid) -> DiscussionTo {
        match self {
            Self::Company => DiscussionTo::Company(object_uuid),
            Self::Component => DiscussionTo::Component(object_uuid),
            Self::Service => DiscussionTo::Service(object_uuid),
        }
    }
}

/// Input arguments for object discussions, used to retrieve discussions associated with a specific object.
///
/// **Enhanced Filtering:**
/// This struct now includes an `object_uuid` for precise object identification and an optional `filter_by_uuids` for further narrowing down discussion results.
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptObjectDiscussionsArg {
    /// Unique identifier of the object associated with the discussions
    object_uuid: Uuid,
    /// Target object type (e.g., company, component, service)
    to_object: ToObject,
    /// Additional filter by UUID to narrow down the list of discussion results
    filter_by_uuids: Option<Vec<Uuid>>,
}

impl IptObjectDiscussionsArg {
    /// Retrieves the discussion target (e.g., company, component, service) associated with the specified object UUID.
    ///
    /// # Returns
    ///
    /// A `DiscussionTo` enum value representing the discussion target, leveraging the provided `object_uuid` for precise identification.
    fn get_discuss_to(&self) -> DiscussionTo {
        self.to_object.discuss_to(self.object_uuid)
    }

    /// Converts the current instance into `DiscussQueryOptions` with the provided sorting and pagination settings.
    pub(crate) fn into_args(
        self,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> DiscussQueryOptions {
        let discussion_to = self.get_discuss_to();
        let s = sort
            .map(|s| {
                Sort::parsing(
                    TableName::DiscussionRef(Some(discussion_to.clone())),
                    &s.by_field,
                    s.as_desc,
                )
            })
            .unwrap_or(Sort::set_by_table(TableName::DiscussionRef(Some(
                discussion_to.clone(),
            ))));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        // Create a new DiscussQueryOptions instance with the determined settings
        DiscussQueryOptions::new(
            discussion_to,
            self.filter_by_uuids.unwrap_or_default(),
            s,
            p,
        )
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptDiscussionArg {
    pub(crate) object_uuid: Uuid,
    pub(crate) to_object: ToObject,
}

impl IptDiscussionArg {
    pub(crate) fn get_discuss_to(&self) -> DiscussionTo {
        self.to_object.discuss_to(self.object_uuid)
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptDiscussionCommentsArg {
    discussion_uuid: Uuid,
    filter_by_parent_uuid: Option<Uuid>,
    filter_by_uuids: Option<Vec<Uuid>>,
}

impl IptDiscussionCommentsArg {
    /// Converts the current instance into `CommentQueryOptions` with the provided sorting and pagination settings.
    pub(crate) fn into_args(
        self,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> CommentQueryOptions {
        let s = sort
            .map(|s| Sort::parsing(TableName::DiscussionCommentList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::DiscussionCommentList));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        CommentQueryOptions::new(
            self.discussion_uuid,
            self.filter_by_parent_uuid,
            self.filter_by_uuids.unwrap_or_default(),
            s,
            p,
        )
    }
}

impl From<Discussion> for DiscussionInfo {
    fn from(mut discussion: Discussion) -> Self {
        DiscussionInfo {
            uuid: discussion.uuid,
            title: std::mem::take(&mut discussion.title),
            is_pinned: discussion.is_pinned,
            last_activity_at: discussion.last_activity_at,
            created_at: discussion.created_at,
        }
    }
}
