use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::search::order::Paginate;
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns standard-related keywords
pub(crate) fn get_standard_keywords(
    logged_user_uuid: &Uuid,
    standard_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_standard_for_user(logged_user_uuid, standard_uuid, &need_access_level, conn)?;

    let keyword_ids = keyword_to_standard::keyword_to_standard
        .filter(keyword_to_standard::standard_uuid.eq(standard_uuid))
        .select(keyword_to_standard::keyword_id)
        .limit(1000)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get keywords for standard: {:?}", err);
            ServiceError::InternalServerError
        })?;
    if keyword_ids.is_empty() {
        return Ok(Vec::new()); // not found keywords
    }
    Keyword::get_by_ids(&keyword_ids, paginate, conn)
}
