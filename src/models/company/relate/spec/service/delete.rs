use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::spec::model::{
    IptCompanySpecsData, DelCompanySpec
};
use crate::models::company::access::util::check_company_access;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_company_specs(
    logged_user_uuid: &Uuid,
    data: &IptCompanySpecsData,
    conn: &PgConnection
) -> ServiceResult<usize> {
    use crate::schema::spec_to_company::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    // creating structures for delete records
    let del_specs: DelCompanySpec = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    diesel::delete(spec_to_company)
        .filter(company_uuid.eq(&del_specs.company_uuid)
        .and(spec_id.eq_any(&del_specs.spec_ids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
