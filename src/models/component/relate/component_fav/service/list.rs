use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::component_fav::model::ComponentFav;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_all_component_favorites(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentFav>> {
    use crate::schema::component_fav::dsl as component_fav;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_fav::component_fav
        .filter(component_fav::uuid_component.eq(&target_uuid_component))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentFav>(conn)?)
}
