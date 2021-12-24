use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::spec::model::{
    IptComponentSpecsData, DeleteComponentSpecs
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_specs(
    logged_user_uuid: &Uuid,
    data: &IptComponentSpecsData,
    conn: &PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // creating structures for delete records
    let del_specs: DeleteComponentSpecs = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    diesel::delete(spec_to_component::spec_to_component)
        .filter(spec_to_component::component_uuid.eq(&del_specs.component_uuid)
        .and(spec_to_component::spec_id.eq_any(&del_specs.spec_ids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
