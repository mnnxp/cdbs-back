use crate::errors::{ServiceResult, ServiceError};
use crate::database::PgConn;
use crate::models::user::model::TargetUser;
use crate::storage::model::UserStorageAccess;
use crate::storage::backblaze::b2_authorize_account::b2_authorize_account;

/// Gets storage of access data in the database
/// or authorization and  update storage of access data in the database
pub(crate) async fn get_user_storage_access(
    target_user: TargetUser,
    pool: PgConn,
) -> ServiceResult<UserStorageAccess> {
    let conn = pool.get().unwrap();

    // find data on database
    let user_storage_access = UserStorageAccess::get(&target_user.0, &conn);

    match user_storage_access {
        Ok(db_data) => Ok(db_data),
        Err(_) => {
            let user_storage_access = UserStorageAccess::get_old(&target_user.0, &conn);
            match user_storage_access {
                Ok(db_data) => {
                    let api_url = db_data.api_url;
                    let application_key_id = db_data.application_key_id;
                    let application_key = db_data.application_key;

                    let new_auth_data = b2_authorize_account(
                        &api_url,
                        &application_key_id,
                        &application_key,
                    ).await;

                    debug!("Authorization: {:#?}", new_auth_data);

                    match new_auth_data {
                        Ok(ref auth_data) => Ok(
                            UserStorageAccess::new_from_auth_data(
                                &target_user.0,
                                auth_data,
                                &conn
                            ).expect("Failed update old storage access data")
                        ),
                        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
                    }
                },
                Err(e) => Err(ServiceError::BadRequest(e.to_string())),
            }
        },
    }
}
