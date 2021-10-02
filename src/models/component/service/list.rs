use crate::errors::ServiceResult;
use crate::models::component::model::{ShowComponentShort, ComponentAndRelatedData};
use crate::models::component::access::util::check_access_component_for_user;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn find_components(
    logged_user_uuid: &Uuid,
    target_components_uuids: &[Uuid],
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    for tcu in target_components_uuids {
        check_access_component_for_user(
            logged_user_uuid,
            tcu,
            &need_access_level,
            true, // ownership_check
            conn
        )?;
    }

    let result: Vec<ShowComponentShort> = ShowComponentShort::get_list_by_uuids(
        target_components_uuids,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list components and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_component_uuid(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ComponentAndRelatedData> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        target_component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    // collect data for component
    let result: ComponentAndRelatedData = ComponentAndRelatedData::collect_related_data(
        target_component_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading component and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
