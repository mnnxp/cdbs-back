use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_by_uuids(
    cxt: &Context<'_>,
    target_uuids_standards: &[Uuid],
    target_user_uuid: &Uuid,
) -> ServiceResult<Vec<ShowStandardShort>> {
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    let result: Vec<ShowStandardShort> = ShowStandardShort::get_list_by_uuids(
        target_uuids_standards,
        target_user_uuid,
        &set_lang_id,
        conn
    ).expect("Error loading list standards and collect short data");

    debug!("Standards data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_by_uuid(
    cxt: &Context<'_>,
    target_standard_uuid: &Uuid,
    target_user_uuid: &Uuid,
) -> ServiceResult<StandardAndRelatedData> {
    let conn: &PooledConnection = &get_conn(cxt)?;

    let set_lang_id = crate::models::user::get_set_language(cxt);

    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_standard_uuid,
        target_user_uuid,
        &set_lang_id,
        conn
    ).expect("Error loading standard and collect related data");

    debug!("Standard data: {:#?}", result);

    Ok(result)
}
