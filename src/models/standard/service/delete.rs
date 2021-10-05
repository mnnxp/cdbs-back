use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::model::SlimStandard;
use crate::models::standard::access::util::check_is_owner;
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete standard with check ownership standard or company
pub(crate) fn del_standard_data(
    logged_user_uuid: &Uuid,
    del_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    // check ownership standard
    if !check_is_owner(
        logged_user_uuid,
        del_standard_uuid,
        conn
    ) {
        // if not ownership standard
        use crate::schema::standard_ref::dsl::*;

        let get_company_uuid = standard_ref
            .filter(uuid.eq(del_standard_uuid))
            .select(company_uuid)
            .get_result::<Uuid>(conn);

        let sd_company_uuid: Uuid = match get_company_uuid {
            Ok(x) => x,
            Err(err) => {
                debug!("Not found standard: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Not found standard".to_string()
                ))
            },
        };

        // check ownership company of standard
        check_is_owner_with_err(
            logged_user_uuid,
            &sd_company_uuid,
            conn
        )?;
    }

    delete_row_standard(
        del_standard_uuid,
        conn
    )
}

/// Delere row about standard in database
fn delete_row_standard(
    del_standard_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    use crate::schema::standard_ref::dsl::*;

    let delete_standard = diesel::delete(standard_ref
        .filter(uuid.eq(del_standard_uuid)))
        .returning((
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            standard_status_id,
        ))
        .get_result::<SlimStandard>(conn);

    match delete_standard {
        Ok(x) => Ok(x),
        Err(err) => {
            debug!("Failed delete standard: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed delete standard".to_string()
            ))
        },
    }
}
