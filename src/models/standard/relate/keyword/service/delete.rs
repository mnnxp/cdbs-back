use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::keyword::model::{DeleteStandardKeywords, IptStandardKeywordsData};
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет ключевые слова из описания стандарта.
pub(crate) fn del_standard_keywords(
    logged_user_uuid: &Uuid,
    data: &IptStandardKeywordsData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        &data.standard_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // creating structures for delete records
    let del_keywords: DeleteStandardKeywords = data.into();

    if del_keywords.keyword_ids.is_empty() {
        // return error if not found correct keywords
        return Err(get_err_msg(ErrorMessage::NotFoundKeywords));
    }

    diesel::delete(keyword_to_standard::keyword_to_standard)
        .filter(
            keyword_to_standard::standard_uuid
                .eq(&del_keywords.standard_uuid)
                .and(keyword_to_standard::keyword_id.eq_any(&del_keywords.keyword_ids)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
