use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::keyword::model::ComponentKeywordsArg;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает ключевые слова, связанные с компонентом.
pub(crate) fn get_component_keywords(
    logged_user_uuid: &Uuid,
    arg: &ComponentKeywordsArg,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Keyword>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &arg.component_uuid,
        &need_access_level,
        conn
    )?;

    let kywords_ids = keyword_to_component::keyword_to_component
        .filter(keyword_to_component::component_uuid.eq(&arg.component_uuid))
        .select(keyword_to_component::keyword_id)
        .limit(arg.limit as i64)
        .offset(arg.offset as i64)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get keywords for component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if kywords_ids.is_empty() {
        return Ok(Vec::new()) // not found keywords
    }

    Keyword::get_by_ids(&kywords_ids, conn)
}
