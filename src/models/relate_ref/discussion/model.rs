use crate::models::search::order::{Paginate, Sort};
use crate::schema::*;
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

use super::util::get_root_discussion_comment_uuid;

#[derive(Debug, Clone)]
pub(crate) enum DiscussionTo {
    Company(Uuid),
    Component(Uuid),
    Service(Uuid),
}

/// Contains information about discussion
#[derive(Serialize, Deserialize, Insertable, Queryable, Clone, Debug, Default)]
#[diesel(table_name = discussion_ref)]
pub(crate) struct Discussion {
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

impl Discussion {
    pub(crate) fn new() -> Self {
        let current_time = Local::now().naive_local();
        Self {
            uuid: Uuid::new_v4(),
            last_activity_at: current_time,
            created_at: current_time,
            ..Default::default()
        }
    }

    pub(crate) fn set(&mut self, title: String, is_pinned: bool) {
        self.title = title;
        self.is_pinned = is_pinned;
    }
}

/// Contains information about discussion comment
#[derive(Serialize, Deserialize, Insertable, Queryable, Clone, Debug, Default)]
#[diesel(table_name = discussion_comment_list)]
pub(crate) struct DiscussionCommentList {
    /// Discussion identifier
    pub(crate) uuid: Uuid,
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Uuid,
    /// Parent comment identifier
    pub(crate) parent_comment_uuid: Uuid,
    /// Identifier of user associated with the comment
    pub(crate) author_uuid: Uuid,
    /// Comment content
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

impl DiscussionCommentList {
    pub(crate) fn from_args(
        author_uuid: Uuid,
        parent_comment_uuid: Option<Uuid>,
        message_content: String,
        discussion_uuid: Uuid,
    ) -> Self {
        let current_time = Local::now().naive_local();
        let parent_comment_uuid = match parent_comment_uuid {
            Some(pcu) => pcu,
            None => get_root_discussion_comment_uuid(),
        };

        Self {
            uuid: Uuid::new_v4(),
            discussion_uuid,
            parent_comment_uuid,
            author_uuid,
            message_content,
            is_edited: false,
            is_hidden: false,
            created_at: current_time,
            updated_at: current_time,
        }
    }
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Clone, Debug, Default)]
#[diesel(table_name = discus_to_company)]
pub(crate) struct DiscusToCompany {
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Uuid,
    /// Identifier of company being discussed
    pub(crate) company_uuid: Uuid,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Clone, Debug, Default)]
#[diesel(table_name = discus_to_component)]
pub(crate) struct DiscusToComponent {
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Uuid,
    /// Identifier of component being discussed
    pub(crate) component_uuid: Uuid,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Clone, Debug, Default)]
#[diesel(table_name = discus_to_service)]
pub(crate) struct DiscusToService {
    /// Discussion thread identifier
    pub(crate) discussion_uuid: Uuid,
    /// Identifier of service being discussed
    pub(crate) service_uuid: Uuid,
}

pub(crate) struct DiscussQueryOptions {
    pub(crate) discussion_to: DiscussionTo,
    pub(crate) discussion_uuids: Vec<Uuid>,
    pub(crate) sort: Sort,
    pub(crate) paginate: Paginate,
}

impl DiscussQueryOptions {
    /// Creates a new instance of `DiscussQueryOptions` from the provided arguments
    pub(crate) fn new(
        discussion_to: DiscussionTo,
        discussion_uuids: Vec<Uuid>,
        sort: Sort,
        paginate: Paginate,
    ) -> Self {
        DiscussQueryOptions {
            discussion_to,
            discussion_uuids,
            sort,
            paginate,
        }
    }
}

pub(crate) struct CommentQueryOptions {
    pub(crate) discussion_uuid: Uuid,
    pub(crate) parent_uuid: Option<Uuid>,
    pub(crate) comment_uuids: Vec<Uuid>,
    pub(crate) sort: Sort,
    pub(crate) paginate: Paginate,
}

impl CommentQueryOptions {
    /// Creates a new instance of `CommentQueryOptions` from the provided arguments.
    pub(crate) fn new(
        discussion_uuid: Uuid,
        parent_uuid: Option<Uuid>,
        comment_uuids: Vec<Uuid>,
        sort: Sort,
        paginate: Paginate,
    ) -> Self {
        CommentQueryOptions {
            discussion_uuid,
            parent_uuid,
            comment_uuids,
            sort,
            paginate,
        }
    }
}