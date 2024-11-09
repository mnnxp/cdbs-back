use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::search::order::Paginate;
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the keywords associated with the component
pub(crate) fn get_component_keywords(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Keyword>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        component_uuid,
        &need_access_level,
        conn
    )?;

    Keyword::for_component_without_check(component_uuid, paginate, conn)
}

impl Keyword {
    /// Returns component keywords without checking access
    pub(crate) fn for_component_without_check(
        component_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection
    ) -> ServiceResult<Vec<Keyword>> {
        let kywords_ids = keyword_to_component::keyword_to_component
            .filter(keyword_to_component::component_uuid.eq(component_uuid))
            .select(keyword_to_component::keyword_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get keywords for component: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if kywords_ids.is_empty() {
            return Ok(Vec::new()) // not found keywords
        }
        Keyword::get_by_ids(&kywords_ids, paginate, conn)
    }
}