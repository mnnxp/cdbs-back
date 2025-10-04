use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::search::order::Paginate;
use crate::models::user::model::{
    IptGetUserArg, ShowUserAndRelatedData, ShowUserShort, SlimUser, UserAndRelatedData, UsersArg,
};
use diesel::PgConnection;
use uuid::Uuid;

/// Возвращает основные и связанные с пользователем данные, по UUID пользователя.
pub(crate) fn get_user_data(
    logged_user_uuid: &Uuid,
    args: &IptGetUserArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<ShowUserAndRelatedData> {
    match (&args.user_uuid, &args.username) {
        (Some(user_uuid), _) => {
            ShowUserAndRelatedData::get_user_by_uuid(logged_user_uuid, user_uuid, set_lang_id, conn)
        }
        (_, Some(username)) => find_user_by_username(logged_user_uuid, username, set_lang_id, conn),
        _ => Err(get_err_msg(ErrorMessage::NeedSetUuidOrUsername)),
    }
}

/// Gets user with related data, with translate by username
pub(crate) fn find_user_by_username(
    logged_user_uuid: &Uuid,
    target_username: &str,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<ShowUserAndRelatedData> {
    use crate::models::user::util::get_uuid_by_username;

    let user_uuid: &Uuid = &get_uuid_by_username(target_username, conn)?;

    ShowUserAndRelatedData::get_user_by_uuid(logged_user_uuid, user_uuid, set_lang_id, conn)
}

/// Возвращает структуру с основной информацией о пользователе (SlimUser)
pub(crate) fn get_self_slim_data(
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<SlimUser> {
    SlimUser::get_by_uuid(logged_user_uuid, conn)
}

/// Возвращает полную информацию об авторизованном пользователе.
pub(crate) fn get_self_user_data(
    logged_user_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<UserAndRelatedData> {
    // collect data for user
    let result: UserAndRelatedData =
        UserAndRelatedData::collect_related_data(logged_user_uuid, set_lang_id, conn)
            .expect("Error loading user and collect related data");
    debug!("Self user data: {:#?}", result);
    Ok(result)
}

/// Returns aggregated user data. Gets a summary of users filtered by:
/// UUID, user (UUID), subscribers, favorites (for yourself).
pub(crate) fn get_users(
    logged_user_uuid: &Uuid,
    arguments: &UsersArg,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowUserShort>> {
    // structure for reduce the number of function arguments
    let UsersArg {
        filter_users_uuids,
        subscribers,
        favorite,
    } = arguments;

    // select target users uuids
    match (subscribers, favorite) {
        // gets users of self subscribers list
        // for authorized user with/without filter
        (true, false) => ShowUserShort::get_followers_by_user_uuid(
            logged_user_uuid,
            filter_users_uuids,
            paginate,
            conn,
        ),
        // gets users of self favorite list
        // for authorized user with/without filter
        (false, true) => ShowUserShort::get_favorites_by_user_uuid(
            logged_user_uuid,
            filter_users_uuids,
            paginate,
            conn,
        ),
        // get all public users
        (false, false) => match filter_users_uuids.is_empty() {
            true => ShowUserShort::get_all_public_users(paginate, conn),
            false => ShowUserShort::get_users_by_uuids(logged_user_uuid, filter_users_uuids, conn),
        },
        (true, true) => Err(get_err_msg(ErrorMessage::FailedMatchArguments)),
    }
}
