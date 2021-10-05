use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Find and check existence user owned component
/// return err if not found file
pub(crate) fn check_file_owner_err(
    user_uuid: &Uuid,
    file_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::file_ref::dsl as file_ref;

    // find file with target user
    let res_search = file_ref::file_ref
        .filter(file_ref::uuid.eq(file_uuid)
        .and(file_ref::user_uuid.eq(user_uuid)))
        .execute(conn);

    match res_search {
        Ok(x) => {
            debug!("Found file: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Not found file: {:?}", err);
            Err(ServiceError::BadRequest(
                "Not found file".to_string()
            ))
        },
    }
}
