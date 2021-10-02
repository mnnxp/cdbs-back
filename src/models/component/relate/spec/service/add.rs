use crate::errors::{
    ServiceResult,
    ServiceError,
};
use crate::models::component::spec::model::{
    ComponentSpec,
    IptComponentSpecData,
    InsertableComponentSpec
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_component_specs(
    logged_user_uuid: &Uuid,
    data: &IptComponentSpecData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::spec_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_kw_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_component_specs: Vec<InsertableComponentSpec> = data.into();

    if new_component_specs.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    let mut insert_data: Vec<InsertableComponentSpec> = Vec::new();

    for component_kw in new_component_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_component
            .filter(component_uuid.eq(&component_kw.component_uuid)
            .and(spec_id.eq(&component_kw.spec_id)))
            .execute(conn).unwrap_or(0);

        if flag_found_spec == 0 {
            debug!("Inserted component spec: {:?}", &component_kw.spec_id);

            insert_data.push(component_kw);

            count_insert_rows += 1;
        } else {
            error_kw_has.push(component_kw.spec_id);
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(ServiceError::BadRequest(
            format!("This ids {:?} already has", error_kw_has)
        ))
    }

    match diesel::insert_into(spec_to_component)
        .values(&insert_data)
        .get_result::<ComponentSpec>(conn) {
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
