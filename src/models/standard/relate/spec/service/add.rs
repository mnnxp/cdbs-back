use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::spec::model::{
    InsertableStandardSpec, IptStandardSpecsData, StandardSpec,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет связи стандарта с разделами каталога.
pub(crate) fn add_standard_specs(
    logged_user_uuid: &Uuid,
    data: &IptStandardSpecsData,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::spec_to_standard::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        &data.standard_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_keywords_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_standard_specs: Vec<InsertableStandardSpec> = data.into();

    if new_standard_specs.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }

    let mut insert_data: Vec<InsertableStandardSpec> = Vec::new();

    for standard_kw in new_standard_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_standard
            .filter(
                standard_uuid
                    .eq(&standard_kw.standard_uuid)
                    .and(spec_id.eq(&standard_kw.spec_id)),
            )
            .execute(conn)
            .map_err(|err| {
                debug!("Failed check spec for standard: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckSpec)
            })?;

        match flag_found_spec {
            0 => {
                debug!("Inserted standard spec: {:?}", &standard_kw.spec_id);
                insert_data.push(standard_kw);
                count_insert_rows += 1;
            }
            _ => error_keywords_has.push(standard_kw.spec_id),
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(get_err_msg(ErrorMessage::IdsAlreadyHas(error_keywords_has)));
    }

    diesel::insert_into(spec_to_standard)
        .values(&insert_data)
        .get_result::<StandardSpec>(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })?;

    debug!("Completed, add {:?} specs", count_insert_rows);
    Ok(count_insert_rows)
}
