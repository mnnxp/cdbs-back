use crate::errors::{
    ServiceResult,
    ServiceError,
};
use crate::models::component::keyword::model::{
    IptComponentKeywordData,
    DeleteComponentKeyword
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_keywords(
    logged_user_uuid: &Uuid,
    data: &IptComponentKeywordData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::keyword_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    // creating structures for delete records
    let del_keywords: DeleteComponentKeyword = data.into();

    if del_keywords.keyword_ids.is_empty() {
        // return error if not found correct keywords
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    match diesel::delete(keyword_to_component)
        .filter(component_uuid.eq(&del_keywords.component_uuid)
        .and(keyword_id.eq_any(&del_keywords.keyword_ids)))
        .execute(conn) {
        Ok(count) => {
            debug!("Completed, delete {:?} keywords", count);
            Ok(count as i32)
        },
        Err(err) => {
            debug!("Fail inserted keyword: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
