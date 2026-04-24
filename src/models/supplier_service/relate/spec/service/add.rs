use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::models::supplier_service::spec::model::{
    InsertableServiceSpec, IptServiceSpecsData, ServiceSpec,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Adds service links to catalog sections
pub(crate) fn add_service_specs(
    data: &IptServiceSpecsData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    use crate::schema::spec_to_service::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Service,
        &data.service_uuid,
        AccessOperation::Write,
        conn,
    )?;

    let mut ok_specs_insert = Vec::new(); // <-- for accumulated count inserted rows
    let mut error_specs_has = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_service_specs: Vec<InsertableServiceSpec> = data.into();

    if new_service_specs.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }

    let mut insert_data: Vec<InsertableServiceSpec> = Vec::new();

    for service_spec in new_service_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_service
            .filter(
                service_uuid
                    .eq(&service_spec.service_uuid)
                    .and(spec_id.eq(&service_spec.spec_id)),
            )
            .execute(conn)
            .map_err(|err| {
                debug!("Failed check spec for service: {:?}", err);
                get_err_msg(ErrorMessage::FailedCheckSpec)
            })?;

        match flag_found_spec {
            0 => {
                ok_specs_insert.push(service_spec.spec_id);
                insert_data.push(service_spec);
            }
            _ => error_specs_has.push(service_spec.spec_id),
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(get_err_msg(ErrorMessage::IdsAlreadyHas(error_specs_has)));
    }

    diesel::insert_into(spec_to_service)
        .values(&insert_data)
        .get_result::<ServiceSpec>(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })?;

    if !ok_specs_insert.is_empty() {
        change_service_updated_at(
            &data.service_uuid,
            logged_user_uuid,
            format!("Added specs {ok_specs_insert:?}, skip with errors {error_specs_has:?}"),
            conn,
        )?;
    }
    Ok(ok_specs_insert.len())
}
