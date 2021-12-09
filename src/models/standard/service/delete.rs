use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::util::check_is_owner;
use crate::models::company::access::util::check_is_owner_with_err;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete standard with check ownership standard or company
pub(crate) fn del_standard_data(
    logged_user_uuid: &Uuid,
    del_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    // check ownership standard
    if !check_is_owner(logged_user_uuid, del_standard_uuid, conn) {
        // if user not ownership standard
        let owner_company_uuid = standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(del_standard_uuid))
            .select(standard_ref::company_uuid)
            .first::<Uuid>(conn)
            .map_err(|err| {
                debug!("Not found standard: {:?}", err);
                ServiceError::BadRequest("Not found standard".to_string())
            })?;

        // check ownership company of standard
        check_is_owner_with_err(
            logged_user_uuid,
            &owner_company_uuid,
            conn
        )?;
    }

    // Delere row about standard in database
    diesel::delete(standard_ref::standard_ref
        .filter(standard_ref::uuid.eq(del_standard_uuid)))
        .returning(standard_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete standard: {:?}", err);
            ServiceError::InternalServerError
        })
}
