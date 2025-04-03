use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::search::order::Paginate;
use crate::schema::keyword_to_service::dsl as keyword_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the keywords associated with the service
pub(crate) fn get_service_keywords(
    service_uuid: &Uuid,
    logged_user_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Keyword>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        service_uuid,
        &need_access_level,
        conn
    )?;

    Keyword::for_service_without_check(service_uuid, paginate, conn)
}

impl Keyword {
    /// Returns service keywords without checking access
    pub(crate) fn for_service_without_check(
        service_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection
    ) -> ServiceResult<Vec<Keyword>> {
        let keyword_ids = keyword_to_service::keyword_to_service
            .filter(keyword_to_service::service_uuid.eq(service_uuid))
            .select(keyword_to_service::keyword_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get keywords for service: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if keyword_ids.is_empty() {
            return Ok(Vec::new()) // not found keywords
        }
        Keyword::get_by_ids(&keyword_ids, paginate, conn)
    }
}