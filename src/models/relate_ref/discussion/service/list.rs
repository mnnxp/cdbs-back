use crate::errors::ServiceResult;
use crate::graphql::discussion_model::{DiscussionCommentData, DiscussionInfo};
use crate::models::relate_ref::discussion::model::{
    CommentQueryOptions, DiscussQueryOptions, Discussion, DiscussionCommentList, DiscussionTo,
};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::objects_order;
use crate::models::user::model::ShowUserShort;
use diesel::prelude::PgConnection;
use uuid::Uuid;

/// Returns a list of available discussion.
pub(crate) fn get_discussions(
    args: &DiscussQueryOptions,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DiscussionInfo>> {
    args.discussion_to
        .check_access(&options.logged_user_uuid, &3, conn)?;
    let mut discussion_uuids = args.discussion_to.get_discuss_uuids(conn)?;
    if discussion_uuids.len() > 1 {
        discussion_uuids = objects_order(&discussion_uuids, &args.sort, &args.paginate, conn)?;
        filter_discussion(&mut discussion_uuids, &args.discussion_uuids);
    }
    Discussion::get_by_uuids(&discussion_uuids, conn)
        .map(|vec_d| vec_d.into_iter().map(DiscussionInfo::from).collect())
}

/// Returns a list of available discussion comment list.
/// If no comments filter is specified, only the first level is sampled (if the parent comment refers to itself).
pub(crate) fn get_discussion_comment_list(
    args: &CommentQueryOptions,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DiscussionCommentData>> {
    let discussion_to = DiscussionTo::by_discuss_uuid(&args.discussion_uuid, conn)?;
    discussion_to.check_access(&options.logged_user_uuid, &3, conn)?;
    let mut res = Vec::new();
    for comment_uuid in &DiscussionCommentList::get_uuids(args, conn)? {
        res.push(get_discuss_comment(comment_uuid, &options.domain, conn)?)
    }
    Ok(res)
}

/// Adds information from other tables to the data
fn get_discuss_comment(
    comment_uuid: &Uuid,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<DiscussionCommentData> {
    let dcl = DiscussionCommentList::get_by_uuid(comment_uuid, conn)?;
    Ok(DiscussionCommentData {
        uuid: dcl.uuid,
        discussion_uuid: dcl.discussion_uuid,
        parent_comment_uuid: dcl.parent_comment_uuid,
        author: ShowUserShort::get_without_check_by_uuid(&dcl.author_uuid, domain, conn)?,
        message_content: dcl.message_content,
        is_edited: dcl.is_edited,
        is_hidden: dcl.is_hidden,
        created_at: dcl.created_at,
        updated_at: dcl.updated_at,
    })
}

/// Filters a list of discussion UUIDs based on a provided filter list.
///
/// **Functionality:**
/// * If the filter list is empty, the function returns `false` without modifying the original list.
/// * Otherwise, it retains only the discussion UUIDs that are present in the filter list.
///
/// # Arguments
///
/// * `discussion_uuids`: The mutable list of discussion UUIDs to be filtered.
/// * `filter_by`: The list of UUIDs to filter by.
///
/// # Returns
///
/// * `true` if the filtering process was successful (i.e., the filter list was not empty).
/// * `false` if the filter list was empty, indicating no filtering was applied.
fn filter_discussion(discussion_uuids: &mut Vec<Uuid>, filter_by: &[Uuid]) -> bool {
    // If the filter list is empty, return false without modifying the original list
    if filter_by.is_empty() {
        return false;
    }
    discussion_uuids.retain_mut(|find_discuss_uuid| {
        filter_by
            .iter()
            .any(|filter_discuss_uuid| filter_discuss_uuid == find_discuss_uuid)
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_filter_discussion_empty_filter_list() {
        // Arrange
        let mut discussion_uuids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let filter_discuss_uuids: Vec<Uuid> = vec![];

        // Act
        let result = filter_discussion(&mut discussion_uuids, &filter_discuss_uuids);

        // Assert
        assert!(result == false);
        assert_eq!(discussion_uuids.len(), 3); // List must not change
    }

    #[test]
    fn test_filter_discussion_non_empty_filter_list() {
        // Arrange
        let mut discussion_uuids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let filter_discuss_uuid = discussion_uuids[1];
        let filter_discuss_uuids = vec![filter_discuss_uuid];

        // Act
        let result = filter_discussion(&mut discussion_uuids, &filter_discuss_uuids);

        // Assert
        assert!(result == true);
        assert_eq!(discussion_uuids.len(), 1); // List must contain only one item
        assert_eq!(discussion_uuids[0], filter_discuss_uuid);
    }

    #[test]
    fn test_filter_discussion_filter_list_with_multiple_matches() {
        // Arrange
        let mut discussion_uuids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let filter_discuss_uuid1 = discussion_uuids[0];
        let filter_discuss_uuid2 = discussion_uuids[2];
        let filter_discuss_uuids = vec![filter_discuss_uuid1, filter_discuss_uuid2];

        // Act
        let result = filter_discussion(&mut discussion_uuids, &filter_discuss_uuids);

        // Assert
        assert!(result == true);
        assert_eq!(discussion_uuids.len(), 2); // List must contain two items
        assert!(discussion_uuids.contains(&filter_discuss_uuid1));
        assert!(discussion_uuids.contains(&filter_discuss_uuid2));
    }

    #[test]
    fn test_filter_discussion_filter_list_with_no_matches() {
        // Arrange
        let mut discussion_uuids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let filter_discuss_uuid = Uuid::new_v4(); // UUID, отсутствующий в discussion_uuids
        let filter_discuss_uuids = vec![filter_discuss_uuid];

        // Act
        let result = filter_discussion(&mut discussion_uuids, &filter_discuss_uuids);

        // Assert
        assert!(result == true);
        assert_eq!(discussion_uuids.len(), 0); // List must be empty
    }
}
