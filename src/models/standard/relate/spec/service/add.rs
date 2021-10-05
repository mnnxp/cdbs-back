use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::spec::model::{
    StandardSpec,
    IptStandardSpecData,
    InsertableStandardSpec
};
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_standard_specs(
    logged_user_uuid: &Uuid,
    data: &IptStandardSpecData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::spec_to_standard::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_kw_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_standard_specs: Vec<InsertableStandardSpec> = data.into();

    if new_standard_specs.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    let mut insert_data: Vec<InsertableStandardSpec> = Vec::new();

    for standard_kw in new_standard_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_standard
            .filter(standard_uuid.eq(&standard_kw.standard_uuid)
            .and(spec_id.eq(&standard_kw.spec_id)))
            .execute(conn);

        match flag_found_spec {
            Ok(x) if x == 0 => {
                debug!("Inserted standard spec: {:?}", &standard_kw.spec_id);

                insert_data.push(standard_kw);

                count_insert_rows += 1;
            },
            Ok(x) => {
                debug!("Found standard spec in database: {:?}", x);

                error_kw_has.push(standard_kw.spec_id);
            },
            Err(err) => {
                debug!("Failed check spec for standard: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed check spec for standard".to_string()
                ))
            },
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(ServiceError::BadRequest(
            format!("This ids {:?} already has", error_kw_has)
        ))
    }

    match diesel::insert_into(spec_to_standard)
        .values(&insert_data)
        .get_result::<StandardSpec>(conn) {
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
