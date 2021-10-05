use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::spec::model::{
    IptStandardSpecData,
    DeleteStandardSpec,
};
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_standard_specs(
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
        conn,
    )?;

    // creating structures for delete records
    let del_specs: DeleteStandardSpec = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    match diesel::delete(spec_to_standard)
        .filter(standard_uuid.eq(&del_specs.standard_uuid)
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
