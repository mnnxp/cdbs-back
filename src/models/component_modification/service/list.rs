use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::models::component_modification::model::ComponentModification;
use diesel::prelude::*;

pub(crate) fn find_all_component_modification(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentModification>> {
    use crate::schema::component_modification_list::dsl::component_modification_list;
    let conn: &PooledConnection = &context.db;

    Ok(component_modification_list
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentModification>(conn)?)
}
