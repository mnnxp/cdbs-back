use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::access::util::check_access_user_for_user;
use crate::models::user::model::{
    SlimUser, ShowUserShort, UserAndRelatedData, ShowUserAndRelatedData
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
            find_user_by_uuid(
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

pub(crate) fn find_users_by_uuids(
    logged_user_uuid: &Uuid,
    target_users_uuids: &[Uuid],
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowUserShort>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for all users
    for tu_uuid in target_users_uuids {
        check_access_user_for_user(
            logged_user_uuid,
            tu_uuid,
            &need_access_level,
            conn
        )?;
    }

    let result: Vec<ShowUserShort> = ShowUserShort::get_list_by_uuids(
        target_users_uuids,
        conn
    ).expect("Error loading list users and collect short data");

    debug!("Users data: {:#?}", result);

    Ok(result)
}

/// Gets user with related data, with translate by uuid
pub(crate) fn find_user_by_uuid(
    logged_user_uuid: &Uuid,
    target_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ShowUserAndRelatedData> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for user
    check_access_user_for_user(
        logged_user_uuid,
        target_user_uuid,
        &need_access_level,
        conn
    )?;

    // collect data for user
    let result: ShowUserAndRelatedData = ShowUserAndRelatedData::collect_related_data(
        target_user_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading user and collect related data");

    debug!("User data: {:#?}", result);

    Ok(result)
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

    find_user_by_uuid(
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
