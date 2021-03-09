use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::models::component::model::Component;
use diesel::prelude::*;
use crate::schema::component_ref::dsl::component_ref;

pub(crate) fn find_all_components(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    use crate::schema::user_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_ref
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Component>(conn)?)
}
