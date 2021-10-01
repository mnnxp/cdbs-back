use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_represent::model::{CompanyRepresent, SlimCompanyRepresent};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn delete_company_represent(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    target_uuid_represent: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyRepresent> {
    use crate::schema::company_represent_ref::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::company::access::util::check_company_access(
        logged_user_uuid,
        target_company_uuid,
        &need_access_level,
        conn,
    )?;

    // debug!("fn target_company_uuid = {}", &target_company_uuid);
    // debug!("fn target_uuid_represent = {}", &target_uuid_represent);

    // find represent and check privileges for delete
    let find_represent = company_represent_ref
        .filter(company_uuid.eq(target_company_uuid))
        .filter(uuid.eq(target_uuid_represent))
        .execute(conn)
        .unwrap_or(0);

    match find_represent as i32 {
        1..=i32::MAX => {
            // delete represent and save delete data for send response
            let delete_company_represent: CompanyRepresent =
                diesel::delete(
                    company_represent_ref.filter(
                        uuid.eq(target_uuid_represent)
                    ))
                    .get_result(conn)?;
            // debug!("fn delete_company_represent ={:?}", &delete_company_represent);
            Ok(delete_company_represent.into())
        },
        _ => Err(ServiceError::BadRequest(
            "The representative not you or not found.".to_string(),
        )),
    }
}
