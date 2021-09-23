use crate::errors::ServiceResult;
use crate::models::component::model::{ShowComponentShort, ComponentAndRelatedData};
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn find_components(
    target_components_uuids: &[Uuid],
    target_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {


    let result: Vec<ShowComponentShort> = ShowComponentShort::get_list_by_uuids(
        target_components_uuids,
        target_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list components and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_component_uuid(
    target_component_uuid: &Uuid,
    target_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ComponentAndRelatedData> {


    // collect data for component
    let result: ComponentAndRelatedData = ComponentAndRelatedData::collect_related_data(
        target_component_uuid,
        target_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading component and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
