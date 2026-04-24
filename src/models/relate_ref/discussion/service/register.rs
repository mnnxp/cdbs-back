use crate::auth::AccessOperation;
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::discussion_model::IptDiscussionCommentData;
use crate::models::relate_ref::discussion::access::CommentCriteria;
use crate::models::relate_ref::discussion::model::{Discussion, DiscussionCommentList};
use crate::schema::discussion_comment_list::dsl as discussion_comment_list;
use crate::schema::discussion_ref::dsl as discussion_ref;
use diesel::prelude::*;
use uuid::Uuid;

use super::update::change_discussion_updated_at;

pub(crate) fn create_discussion_comment(
    logged_user_uuid: &Uuid,
    data: &IptDiscussionCommentData,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // data validation
    if data.message_content.is_empty() {
        return Err(get_err_msg(ErrorMessage::DataNotFound));
    }
    if data.message_content.len() > 5000 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(5000)));
    }
    let mut discussion_to = data.object_discussion.get_discuss_to();
    discussion_to.check_access(logged_user_uuid, AccessOperation::Read, conn)?;
    let valid_discussion_uuid =
        match discussion_to.get_associated_discussion_uuid(&data.discussion_uuid, conn) {
            Ok(d_uuid) => d_uuid,
            Err(err) => {
                debug!(
                    "Failed valid discussion data: {:?}, discussion_uuid of data: {:?}",
                    err, data.discussion_uuid
                );
                if data.discussion_uuid.is_some() {
                    return Err(err);
                }
                let new_discussion_uuid =
                    create_discussion_without_check(String::from("First"), true, conn)?;
                discussion_to.relate_discussion_to_object(&new_discussion_uuid, conn)?;
                debug!("New discussion has been created: {:?}", new_discussion_uuid);
                new_discussion_uuid
            }
        };
    let mut valid_parent_comment_op = None;
    if let Some(parent_comment_uuid) = data.parent_comment_uuid {
        let dc_data = DiscussionCommentList::get_by_uuid(&parent_comment_uuid, conn)?;
        debug!("Parent's comment found {:?}", dc_data);
        if valid_discussion_uuid == dc_data.discussion_uuid {
            valid_parent_comment_op = Some(parent_comment_uuid);
        } else {
            debug!(
                "Parent's comment {:?} not found in the discussion {:?}",
                parent_comment_uuid, valid_discussion_uuid
            );
            return Err(get_err_msg(ErrorMessage::FailedMatchArguments));
        }
    }
    let insert_comment = DiscussionCommentList::from_args(
        *logged_user_uuid,
        data.parent_comment_uuid,
        data.message_content.clone(),
        valid_discussion_uuid,
    );
    CommentCriteria::new(&insert_comment.uuid).is_comment_duplicate(
        &insert_comment.discussion_uuid,
        valid_parent_comment_op,
        &insert_comment.author_uuid,
        &insert_comment.message_content,
        conn,
    )?;
    let new_comment_uuid = diesel::insert_into(discussion_comment_list::discussion_comment_list)
        .values(insert_comment)
        .returning(discussion_comment_list::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed create discussion comment: {:?}", err);
            ServiceError::InternalServerError
        })?;
    if valid_parent_comment_op.is_none() {
        set_self_parent_comment(&new_comment_uuid, conn)?;
    }
    change_discussion_updated_at(&valid_discussion_uuid, conn)?;
    Ok(new_comment_uuid)
}

fn create_discussion_without_check(
    title: String,
    is_pinned: bool,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    let mut insert_discussion = Discussion::new();
    insert_discussion.set(title, is_pinned);
    diesel::insert_into(discussion_ref::discussion_ref)
        .values(insert_discussion)
        .returning(discussion_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed create discussion: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Sets the comment Uuid to reference the parent comment (so that it references itself).
fn set_self_parent_comment(comment_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<usize> {
    diesel::update(
        discussion_comment_list::discussion_comment_list
            .filter(discussion_comment_list::uuid.eq(comment_uuid)),
    )
    .set(discussion_comment_list::parent_comment_uuid.eq(comment_uuid))
    .execute(conn)
    .map_err(|err| {
        debug!("Failed to set uuid of parent comment to itself: {:?}", err);
        get_err_msg(ErrorMessage::FailedUpdateData)
    })
}
