use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::search::order::Paginate;
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
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        standard_uuid,
        AccessOperation::Read,
        conn,
    )?;

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
