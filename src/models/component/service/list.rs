use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::model::{ShowComponentShort, ComponentAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_components(
    context: &Context<'_>,
    target_components_uuids: &[Uuid],
    target_user_uuid: &Uuid,
) -> ServiceResult<Vec<ShowComponentShort>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let result: Vec<ShowComponentShort> = ShowComponentShort::get_list_by_uuids(
        target_components_uuids,
        target_user_uuid,
        &set_id_lang,
        conn
    ).expect("Error loading list components and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_uuid_component(
    context: &Context<'_>,
    target_component_uuid: &Uuid,
    target_user_uuid: &Uuid,
) -> ServiceResult<ComponentAndRelatedData> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for component
    let result: ComponentAndRelatedData = ComponentAndRelatedData::collect_related_data(
        target_component_uuid,
        target_user_uuid,
        &set_id_lang,
        conn
    ).expect("Error loading component and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
