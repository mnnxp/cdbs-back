use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::models::supplier_service::spec::model::{DeleteServiceSpecs, IptServiceSpecsData};
use crate::schema::spec_to_service::dsl as spec_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Removes service linkage to catalog sections
pub(crate) fn del_service_specs(
    data: &IptServiceSpecsData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn,
    )?;

    // creating structures for delete records
    let del_specs: DeleteServiceSpecs = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }
    change_service_updated_at(
        &data.service_uuid,
        logged_user_uuid,
        format!("Deleted the categories (specs): {:?}", del_specs.spec_ids),
        conn,
    )?;
    diesel::delete(spec_to_service::spec_to_service)
        .filter(
            spec_to_service::service_uuid
                .eq(&del_specs.service_uuid)
                .and(spec_to_service::spec_id.eq_any(&del_specs.spec_ids)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
