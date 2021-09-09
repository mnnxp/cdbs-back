use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::model::{ShowUserShort, UserAndRelatedData};
use async_graphql::Context;
use uuid::Uuid;

pub(crate) fn find_users_by_uuids(
    context: &Context<'_>,
    target_users_uuids: &[Uuid],
) -> ServiceResult<Vec<ShowUserShort>> {
    let conn: &PooledConnection = &get_conn(context)?;

    let result: Vec<ShowUserShort> = ShowUserShort::get_list_by_uuids(
        target_users_uuids,
        conn
    ).expect("Error loading list users and collect short data");

    debug!("Users data: {:#?}", result);

    Ok(result)
}

/// Gets user with related data, with translate by uuid
pub(crate) fn find_user_by_uuid(
    context: &Context<'_>,
    target_user_uuid: &Uuid,
    logged_user_uuid: &Uuid,
) -> ServiceResult<UserAndRelatedData> {
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for user
    let result: UserAndRelatedData = UserAndRelatedData::collect_related_data(
        target_user_uuid,
        logged_user_uuid,
        &set_id_lang,
        conn
    ).expect("Error loading user and collect related data");

    debug!("User data: {:#?}", result);

    Ok(result)
}
