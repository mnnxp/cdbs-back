use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::{
    SlimUser, ShowUserShort, UserAndRelatedData, ShowUserAndRelatedData, UsersArg,
};
use diesel::PgConnection;
use uuid::Uuid;

/// Gets user data by uuid or username
pub(crate) fn get_user_data (
    logged_user_uuid: &Uuid,
    user_uuid: &Option<Uuid>,
    username: &Option<String>,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ShowUserAndRelatedData> {
    match (user_uuid, username) {
        (Some(ref uu), _) => {
            ShowUserAndRelatedData::get_user_by_uuid(
                logged_user_uuid,
                uu,
                set_lang_id,
                conn,
            )
        },
        (_, Some(ref un)) => {
            find_user_by_username(
                logged_user_uuid,
                un,
                set_lang_id,
                conn,
            )
        },
        _ => {
            Err(ServiceError::BadRequest(
                "Need set userUuid or username".to_string()
            ))
        },
    }
}

/// Gets user with related data, with translate by username
pub(crate) fn find_user_by_username(
    logged_user_uuid: &Uuid,
    target_username: &str,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ShowUserAndRelatedData> {
    use crate::models::user::util::get_uuid_by_username;

    let user_uuid: &Uuid = &get_uuid_by_username(
        target_username,
        conn
    )?;

    ShowUserAndRelatedData::get_user_by_uuid(
        logged_user_uuid,
        user_uuid,
        set_lang_id,
        conn
    )
}

/// Gets slim data logged user
pub(crate) fn get_self_slim_data(
    logged_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<SlimUser> {
    SlimUser::get_by_uuid(logged_user_uuid, conn)
}

/// Gets self (logged user) with related data, with translate
pub(crate) fn get_self_user_data(
    logged_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<UserAndRelatedData> {
    // collect data for user
    let result: UserAndRelatedData = UserAndRelatedData::collect_related_data(
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading user and collect related data");

    debug!("Self user data: {:#?}", result);

    Ok(result)
}

/// Gets user short data with filter by:
/// uuids, favorite list
pub(crate) fn get_users(
    logged_user_uuid: &Uuid,
    arguments: &UsersArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowUserShort>> {
    // structure for reduce the number of function arguments
    let UsersArg {
        filter_users_uuids,
        subscribers,
        favorite,
        limit,
        offset,
    } = arguments;

    // select target users uuids
    match (subscribers, favorite) {
        // gets users of self subscribers list
        // for authorized user with/without filter
        (true, false) => {
            ShowUserShort::get_followers_by_user_uuid(
                logged_user_uuid,
                filter_users_uuids,
                limit,
                offset,
                conn
            )
        },
        // gets users of self favorite list
        // for authorized user with/without filter
        (false, true) => {
            ShowUserShort::get_favorites_by_user_uuid(
                logged_user_uuid,
                filter_users_uuids,
                limit,
                offset,
                conn
            )
        },
        // get all public users
        (false, false) => {
            match filter_users_uuids.is_empty() {
                true => {
                    ShowUserShort::get_all_public_users(
                        limit,
                        offset,
                        conn
                    )
                },
                false => {
                    ShowUserShort::get_users_by_uuids(
                        logged_user_uuid,
                        filter_users_uuids,
                        conn
                    )
                },
            }

        },
        (true, true) => {
            Err(ServiceError::BadRequest(
                "Failed match arguments".to_string()
            ))
        },
    }
}
