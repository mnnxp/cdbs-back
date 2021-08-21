use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::keyword::model::Keyword;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_all_component_keywords(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Keyword>> {
    use crate::schema::keyword_to_component::dsl as keyword_to_component;
    use crate::schema::keyword_ref::dsl as keyword_ref;
    let conn: &PooledConnection = &get_conn(context)?;

    let id_keyword_for_uuid: Vec<i32> = keyword_to_component::keyword_to_component
        .filter(keyword_to_component::uuid_component.eq(target_uuid_component))
        .select(keyword_to_component::id_keyword)
        .load::<i32>(conn)?;

    Ok(keyword_ref::keyword_ref
        .filter(keyword_ref::id.eq_any(id_keyword_for_uuid))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Keyword>(conn)?)
}
