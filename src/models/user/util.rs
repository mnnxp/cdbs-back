use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

/// Gets user_uuid by username
/// return ServiceError(Not found) if have err
pub(crate) fn get_uuid_by_username(
    username: &str,
    conn: &PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::user_ref::dsl as user_ref;

    let res = user_ref::user_ref
        .filter(user_ref::username.eq(username))
        .select(user_ref::uuid)
        .first::<Uuid>(conn);

    match res {
        Ok(value) => Ok(value),
        Err(err) => {
            debug!("Failed get user_uuid by username: {:?}", err);
            Err(ServiceError::BadRequest(
                "Data not found".to_string()
            ))
        },
    }
}
