use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::spec::model::{
    IptStandardSpecsData, DeleteStandardSpecs,
};
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::spec_to_standard::dsl as spec_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет связь стандарта с разделами каталога.
pub(crate) fn del_standard_specs(
    logged_user_uuid: &Uuid,
    data: &IptStandardSpecsData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn,
    )?;

    // creating structures for delete records
    let del_specs: DeleteStandardSpecs = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(ServiceError::BadRequest("Not found specs".to_string()))
    }

    diesel::delete(spec_to_standard::spec_to_standard)
        .filter(spec_to_standard::standard_uuid.eq(&del_specs.standard_uuid)
        .and(spec_to_standard::spec_id.eq_any(&del_specs.spec_ids)))
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
