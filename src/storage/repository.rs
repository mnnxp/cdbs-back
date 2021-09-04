use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::UserStorageAccess;
use crate::storage::backblaze::b2_types::AuthorizeAccountData;
use crate::schema::user_storage_access_ref::dsl as user_storage_access_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl UserStorageAccess {
    /// Saves user storage access data to database
    /// If there is data: updating api_url, authorization_token, token_expiration_at;
    /// If there is no data: inserting new record from updated_storage_access;
    pub(crate) fn new(
        new_storage_access: &UserStorageAccess,
        conn: &PgConnection,
    ) -> ServiceResult<UserStorageAccess> {
        // search storage access data for target user
        let check_data = user_storage_access_ref::user_storage_access_ref
            .filter(user_storage_access_ref::uuid_user.eq(&new_storage_access.uuid_user))
            // .and(user_storage_access_ref::application_key_id.eq(&new_storage_access.application_key_id))
            // .and(user_storage_access_ref::application_key.eq(&new_storage_access.application_key)))
            .execute(conn)
            .expect("Failed check storage access data");

        match check_data as i32 {
            // there is no data: inserting new record from updated_storage_access
            0 => {
                Ok(diesel::insert_into(user_storage_access_ref::user_storage_access_ref)
                    .values(new_storage_access)
                    .get_result::<UserStorageAccess>(conn)
                    .expect("Failed save storage access data"))
            },
            // there is data: updating api_url, authorization_token, token_expiration_at
            1 => {
                Ok(diesel::update(user_storage_access_ref::user_storage_access_ref
                        .filter(user_storage_access_ref::uuid_user.eq(&new_storage_access.uuid_user))
                    ).set((
                        user_storage_access_ref::application_key_id.eq(&new_storage_access.application_key_id),
                        user_storage_access_ref::application_key.eq(&new_storage_access.application_key),
                        user_storage_access_ref::authorization_token.eq(&new_storage_access.authorization_token),
                        user_storage_access_ref::token_expiration_at.eq(&new_storage_access.token_expiration_at)
                    ))
                    .get_result::<UserStorageAccess>(conn)
                    .expect("Failed update storage access data"))
            },
            // more duplicate keys
            _ => {
                diesel::delete(user_storage_access_ref::user_storage_access_ref
                    .filter(user_storage_access_ref::uuid_user.eq(&new_storage_access.uuid_user)))
                    .execute(conn)
                    .expect("Failed delete duplicate storage access data");
                Err(ServiceError::BadRequest("Access storage data broken".to_string()))
            },
        }
    }

    /// Updating user storage access data to database
    /// from api_url, authorization_token, token_expiration_at
    pub(crate) fn new_from_auth_data(
        target_uuid_user: &Uuid,
        new_auth_data: &AuthorizeAccountData,
        conn: &PgConnection,
    ) -> ServiceResult<UserStorageAccess> {
        let naive_local_now = chrono::Local::now().naive_local();

        let check_data = user_storage_access_ref::user_storage_access_ref
            .filter(user_storage_access_ref::uuid_user.eq(&target_uuid_user)
            .and(user_storage_access_ref::key_expiration_at.gt(naive_local_now)))
            .execute(conn)
            .expect("Failed check storage access data for update");

        match check_data as i32 {
            // there is no data: inserting new record from updated_storage_access
            0 => Err(ServiceError::BadRequest("Access storage data broken".to_string())),
            // there is data: updating api_url, authorization_token, token_expiration_at
            1 => {
                let naive_add_one_day = chrono::Local::now().naive_local()+chrono::Duration::days(1);

                Ok(diesel::update(user_storage_access_ref::user_storage_access_ref
                        .filter(user_storage_access_ref::uuid_user.eq(&target_uuid_user))
                    ).set((
                        user_storage_access_ref::api_url.eq(&new_auth_data.api_url),
                        user_storage_access_ref::authorization_token.eq(&new_auth_data.authorization_token),
                        user_storage_access_ref::token_expiration_at.eq(naive_add_one_day)
                    ))
                    .get_result::<UserStorageAccess>(conn)
                    .expect("Failed update storage access data"))
            },
            // more duplicate keys
            _ => Err(ServiceError::BadRequest("Access storage data broken".to_string())),
        }
    }

    /// Gets user storage access data from database
    pub(crate) fn get(
        target_uuid_user: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserStorageAccess> {
        let naive_local_now = chrono::Local::now().naive_local();

        let access_data = user_storage_access_ref::user_storage_access_ref
            .filter(user_storage_access_ref::uuid_user.eq(&target_uuid_user)
            .and(user_storage_access_ref::key_expiration_at.gt(naive_local_now))
            .and(user_storage_access_ref::token_expiration_at.gt(naive_local_now)))
            .first::<UserStorageAccess>(conn);

        match access_data {
            Ok(data) => {
                debug!("Access data: {:#?}", &data);
                Ok(data)
            },
            Err(e) => {
                debug!("Failed get storage access data: {:#?}", e);
                Err(ServiceError::BadRequest("Failed get storage access data".to_string()))
            },
        }
            // more duplicate keys
            // _ => Err(ServiceError::BadRequest("Access storage data broken".to_string())),
    }

    /// Gets user storage old access data for update
    pub(crate) fn get_old(
        target_uuid_user: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserStorageAccess> {
        let access_data = user_storage_access_ref::user_storage_access_ref
            .filter(user_storage_access_ref::uuid_user.eq(&target_uuid_user))
            .first::<UserStorageAccess>(conn);

        match access_data {
            Ok(data) => {
                debug!("Old access data: {:#?}", &data);
                Ok(data)
            },
            Err(e) => {
                debug!("Failed get storage old access data: {:#?}", e);
                Err(ServiceError::BadRequest("Failed get storage old access data".to_string()))
            },
        }
    }
}
