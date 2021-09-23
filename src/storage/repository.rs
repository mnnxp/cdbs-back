use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::StorageAccess;
use crate::schema::storage_access_ref::dsl as storage_access_ref;
use diesel::prelude::*;
// use uuid::Uuid;

impl StorageAccess {
    /// Gets user storage access data from database
    pub(crate) fn get(
        // target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<StorageAccess> {
        // let naive_local_now = chrono::Local::now().naive_local();

        let access_data = storage_access_ref::storage_access_ref
            // .filter(storage_access_ref::expiration_at.gt(naive_local_now))
            .first::<StorageAccess>(conn);

        match access_data {
            Ok(data) => {
                // debug!("Access data: {:#?}", &data);
                Ok(data)
            },
            Err(e) => {
                debug!("Failed get storage access data: {:#?}", e);
                Err(ServiceError::BadRequest("Failed get storage access data. Please tell administration to update storage access.".to_string()))
            },
        }
    }
}
