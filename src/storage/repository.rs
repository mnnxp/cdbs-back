use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::StorageAccess;
use crate::schema::storage_access_ref::dsl as storage_access_ref;
use diesel::prelude::*;
// use uuid::Uuid;

impl StorageAccess {
    /// Gets user storage access data from database
    pub(crate) fn get(
        conn: &PgConnection,
    ) -> ServiceResult<StorageAccess> {
        storage_access_ref::storage_access_ref
            .first::<StorageAccess>(conn)
            .map_err(|err| {
                debug!("Failed get storage access data: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
