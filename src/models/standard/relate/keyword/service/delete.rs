use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::keyword::model::{
    IptStandardKeywordsData,
    DeleteStandardKeywords,
};
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_standard_keywords(
    logged_user_uuid: &Uuid,
    data: &IptStandardKeywordsData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::keyword_to_standard::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn,
    )?;

    // creating structures for delete records
    let del_keywords: DeleteStandardKeywords = data.into();

    if del_keywords.keyword_ids.is_empty() {
        // return error if not found correct keywords
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    match diesel::delete(keyword_to_standard)
        .filter(standard_uuid.eq(&del_keywords.standard_uuid)
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
