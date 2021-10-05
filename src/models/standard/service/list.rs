use crate::errors::ServiceResult;
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn find_by_uuids(
    logged_user_uuid: &Uuid,
    target_standards_uuids: &[Uuid],
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowStandardShort>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    for tsd in target_standards_uuids {
        check_access_standard_for_user(
            logged_user_uuid,
            tsd,
            &need_access_level,
            conn
        )?;
    }

    let result: Vec<ShowStandardShort> = ShowStandardShort::get_list_by_uuids(
        target_standards_uuids,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list standards and collect short data");

    debug!("Standards data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_by_uuid(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<StandardAndRelatedData> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        target_standard_uuid,
        &need_access_level,
        conn
    )?;

    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_standard_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading standard and collect related data");

    debug!("Standard data: {:#?}", result);

    Ok(result)
}
