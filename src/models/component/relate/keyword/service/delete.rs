use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::keyword::model::{
    IptComponentKeywordsData, DeleteComponentKeyword
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_keywords(
    logged_user_uuid: &Uuid,
    data: &IptComponentKeywordsData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // creating structures for delete records
    let del_keywords: DeleteComponentKeyword = data.into();

    if del_keywords.keyword_ids.is_empty() {
        // return error if not found correct keywords
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    diesel::delete(keyword_to_component::keyword_to_component)
        .filter(keyword_to_component::component_uuid.eq(&del_keywords.component_uuid)
        .and(keyword_to_component::keyword_id.eq_any(&del_keywords.keyword_ids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
