use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::keyword::model::StandardKeywordsArg;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Get all keywords for standard
pub(crate) fn get_standard_keywords(
    logged_user_uuid: &Uuid,
    arg: &StandardKeywordsArg,
    conn: &PgConnection
) -> ServiceResult<Vec<Keyword>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &arg.standard_uuid,
        &need_access_level,
        conn
    )?;

    let kywords_ids = keyword_to_standard::keyword_to_standard
        .filter(keyword_to_standard::standard_uuid.eq(&arg.standard_uuid))
        .select(keyword_to_standard::keyword_id)
        .limit(arg.limit as i64)
        .offset(arg.offset as i64)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get keywords for standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if kywords_ids.is_empty() {
        return Ok(Vec::new()) // not found keywords
    }

    Keyword::get_by_ids(
        &kywords_ids,
        conn
    )
}
