use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_by_uuids(
    context: &Context<'_>,
    target_uuids_standards: &[Uuid],
    target_uuid_user: &Uuid,
) -> ServiceResult<Vec<ShowStandardShort>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let result: Vec<ShowStandardShort> = ShowStandardShort::get_list_by_uuids(
        target_uuids_standards,
        target_uuid_user,
        &set_id_lang,
        conn
    ).expect("Error loading list standards and collect short data");

    debug!("Standards data: {:#?}", result);

    Ok(result)
}

pub(crate) fn find_by_uuid(
    context: &Context<'_>,
    target_uuid_standard: &Uuid,
    target_uuid_user: &Uuid,
) -> ServiceResult<StandardAndRelatedData> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_uuid_standard,
        target_uuid_user,
        &set_id_lang,
        conn
    ).expect("Error loading standard and collect related data");

    debug!("Standard data: {:#?}", result);

    Ok(result)
}
