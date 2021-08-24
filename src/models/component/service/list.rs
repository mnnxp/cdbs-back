use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::model::{ShowComponentShort, ComponentAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_components(
    context: &Context<'_>,
    target_uuids_components: Vec<Uuid>,
    target_uuid_user: Uuid,
) -> ServiceResult<Vec<ShowComponentShort>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let result: Vec<ShowComponentShort> = ShowComponentShort::get_list_by_uuids(
        &target_uuids_components,
        &target_uuid_user,
        &set_id_lang,
        conn
    ).expect("Error loading list components and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
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
