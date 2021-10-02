use crate::errors::{
    ServiceResult,
    ServiceError,
};
use crate::models::component::spec::model::{
    IptComponentSpecData,
    DeleteComponentSpec
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_specs(
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

    // creating structures for delete records
    let del_specs: DeleteComponentSpec = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    match diesel::delete(spec_to_component)
        .filter(component_uuid.eq(&del_specs.component_uuid)
        .and(spec_id.eq_any(&del_specs.spec_ids)))
        .execute(conn) {
        Ok(count) => {
            debug!("Completed, delete {:?} specs", count);

            Ok(count as i32)
        },
        Err(err) => {
            debug!("Fail inserted spec: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
