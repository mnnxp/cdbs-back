use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::spec::model::{
    IptComponentSpecsData, InsertableComponentSpec
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет связи компонента с разделами каталога.
pub(crate) fn add_component_specs(
    logged_user_uuid: &Uuid,
    data: &IptComponentSpecsData,
    conn: &mut PgConnection
) -> ServiceResult<i32> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_kw_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_component_specs: Vec<InsertableComponentSpec> = data.into();

    if new_component_specs.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs))
    }

    let mut insert_data: Vec<InsertableComponentSpec> = Vec::new();

    for component_kw in new_component_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_component::spec_to_component
            .filter(spec_to_component::component_uuid.eq(&component_kw.component_uuid)
            .and(spec_to_component::spec_id.eq(&component_kw.spec_id)))
            .execute(conn)
            .map_err(|err| {
                debug!("Fail count specs: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match flag_found_spec {
            0 => {
                debug!("Inserted component spec: {:?}", &component_kw.spec_id);
                insert_data.push(component_kw);
                count_insert_rows += 1;
            },
            _ => error_kw_has.push(component_kw.spec_id),
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(get_err_msg(ErrorMessage::IdsAlreadyHas(error_kw_has)))
    }

    diesel::insert_into(spec_to_component::spec_to_component)
        .values(&insert_data)
        .returning(spec_to_component::spec_id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count_insert_rows)
}
