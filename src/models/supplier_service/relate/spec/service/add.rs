use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::supplier_service::spec::model::{
    ServiceSpec, IptServiceSpecsData, InsertableServiceSpec
};
use crate::models::supplier_service::access::util::check_access_service_for_user;
use diesel::prelude::*;
use uuid::Uuid;

/// Adds service links to catalog sections
pub(crate) fn add_service_specs(
    data: &IptServiceSpecsData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    use crate::schema::spec_to_service::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_keywords_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_service_specs: Vec<InsertableServiceSpec> = data.into();

    if new_service_specs.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs))
    }

    let mut insert_data: Vec<InsertableServiceSpec> = Vec::new();

    for service_kw in new_service_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_service
            .filter(service_uuid.eq(&service_kw.service_uuid)
            .and(spec_id.eq(&service_kw.spec_id)))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed check spec for service: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckSpec)
            })?;

        match flag_found_spec {
            0 => {
                debug!("Inserted service spec: {:?}", &service_kw.spec_id);
                insert_data.push(service_kw);
                count_insert_rows += 1;
            },
            _ => error_keywords_has.push(service_kw.spec_id),
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(get_err_msg(ErrorMessage::IdsAlreadyHas(error_keywords_has)))
    }

    diesel::insert_into(spec_to_service)
        .values(&insert_data)
        .get_result::<ServiceSpec>(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })?;

    debug!("Completed, add {:?} specs", count_insert_rows);
    Ok(count_insert_rows)
}
