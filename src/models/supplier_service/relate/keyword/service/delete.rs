use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::supplier_service::keyword::model::{
    DeleteServiceKeyword, IptServiceKeywordsData,
};
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::schema::keyword_to_service::dsl as keyword_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет ключевые слова из компонента.
pub(crate) fn del_service_keywords(
    data: &IptServiceKeywordsData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Service,
        &data.service_uuid,
        AccessOperation::Write,
        conn,
    )?;

    // creating structures for delete records
    let del_keywords: DeleteServiceKeyword = data.into();

    if del_keywords.keyword_ids.is_empty() {
        // return error if not found correct keywords
        return Err(get_err_msg(ErrorMessage::NotFoundKeywords));
    }
    change_service_updated_at(
        &data.service_uuid,
        logged_user_uuid,
        format!("Deleted the keyword ids: {:?}", &del_keywords.keyword_ids),
        conn,
    )?;
    diesel::delete(keyword_to_service::keyword_to_service)
        .filter(
            keyword_to_service::service_uuid
                .eq(&del_keywords.service_uuid)
                .and(keyword_to_service::keyword_id.eq_any(&del_keywords.keyword_ids)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
