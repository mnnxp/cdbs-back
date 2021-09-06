use crate::errors::ServiceResult;
use crate::database::PgConn;
use crate::models::user::model::TargetUser;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::create_access_data::get_new_storage_access;
use crate::storage::wrapper::authorize_account::update_authorized_storage;

/// Gets valid storage access for user in database
/// if not found, trying get new auth or create new key
pub(crate) async fn get_user_storage_access(
    target_user: TargetUser,
    pool: PgConn,
) -> ServiceResult<UserStorageAccess> {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    // search for the valid token for user in the database
    let mut user_storage_access = UserStorageAccess::get(&target_user.0, &conn);

    // if not found valid access data in the database,
    if user_storage_access.is_err() {
        // requested generate new token is  for key
        user_storage_access = update_authorized_storage (target_user.clone(), pool.clone()).await;
    }

    // if the access data could not be found in the database,
    if user_storage_access.is_err() {
        // is requested new key for user
        user_storage_access = get_new_storage_access(target_user, pool).await;
    }

    match user_storage_access {
        // data for update received
        Ok(new_access) => Ok(new_access),
        // fatality error
        Err(e) => Err(e),
    }
}
