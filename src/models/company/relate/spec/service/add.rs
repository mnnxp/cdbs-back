use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::spec::model::{
    CompanySpec, IptCompanySpecData, InsertableCompanySpec
};
use crate::models::company::access::util::check_company_access;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_company_specs(
    logged_user_uuid: &Uuid,
    data: &IptCompanySpecData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::spec_to_company::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_sc_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_company_specs: Vec<InsertableCompanySpec> = data.into();

    if new_company_specs.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    let mut insert_data: Vec<InsertableCompanySpec> = Vec::new();

    for company_sc in new_company_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_company
            .filter(company_uuid.eq(&company_sc.company_uuid)
            .and(spec_id.eq(&company_sc.spec_id)))
            .execute(conn).unwrap_or(0);

        if flag_found_spec == 0 {
            debug!("Inserted company spec: {:?}", &company_sc.spec_id);

            insert_data.push(company_sc);

            count_insert_rows += 1;
        } else {
            error_sc_has.push(company_sc.spec_id);
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(ServiceError::BadRequest(
            format!("This ids {:?} already has", error_sc_has)
        ))
    }

    match diesel::insert_into(spec_to_company)
        .values(&insert_data)
        .get_result::<CompanySpec>(conn) {
        Ok(_) => {
            debug!("Completed, add {:?} specs", count_insert_rows);
            Ok(count_insert_rows)
        },
        Err(err) => {
            debug!("Fail inserted spec: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
