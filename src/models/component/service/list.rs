use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::model::{Component, ComponentAndRelatedData};
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_components(
    context: &Context<'_>,
    target_uuid_component: Vec<Uuid>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    use crate::schema::component_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let mut query = component_ref.into_boxed();

    if !target_uuid_component.is_empty() {
        query = query.filter(uuid.eq_any(target_uuid_component))
    }

    Ok(query
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Component>(conn)?)
}

pub(crate) fn find_uuid_component(
    context: &Context<'_>,
    target_uuid_component: Uuid,
) -> ServiceResult<ComponentAndRelatedData> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for component
    let result: ComponentAndRelatedData = ComponentAndRelatedData::collect_related_data(
        &target_uuid_component,
        &set_id_lang,
        conn
    ).expect("Error loading component and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
