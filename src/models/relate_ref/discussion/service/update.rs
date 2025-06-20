use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::discussion_model::IptEditCommentData;
use crate::models::relate_ref::discussion::access::CommentCriteria;
use crate::schema::discussion_ref::dsl as discussion_ref;
use crate::schema::discussion_comment_list::dsl as discussion_comment_list;
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn edit_discussion_comment(
    logged_user_uuid: &Uuid,
    data: &IptEditCommentData,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    let comment_criteria = CommentCriteria::new(&data.comment_uuid);
    // check access
    comment_criteria.verify_comment_ownership(logged_user_uuid, conn)?;
    // data validation
    if data.updated_message.is_empty() {
        return Err(get_err_msg(ErrorMessage::DataNotFound));
    }
    if data.updated_message.len() > 4000 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(4000)));
    }
    comment_criteria.is_comment_message_present(&data.updated_message, conn)?;
    // update data
    let discussion_uuid = diesel::update(discussion_comment_list::discussion_comment_list)
        .filter(discussion_comment_list::uuid.eq(&data.comment_uuid)
        .and(discussion_comment_list::message_content.ne(&data.updated_message)))
        .set((
            discussion_comment_list::message_content.eq(&data.updated_message),
            discussion_comment_list::updated_at.eq(&Local::now().naive_local())
        ))
        .returning(discussion_comment_list::discussion_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed create discussion comment: {:?}", err);
            ServiceError::InternalServerError
        })?;
    change_discussion_updated_at(&discussion_uuid, conn)?;
    Ok(true)
}

/// Sets current time as value updated at for target discussion
pub(crate) fn change_discussion_updated_at(
    discussion_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    diesel::update(discussion_ref::discussion_ref
        .filter(discussion_ref::uuid.eq(discussion_uuid)))
        .set(discussion_ref::last_activity_at.eq(Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })
}