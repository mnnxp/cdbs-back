use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::spec::model::{DeleteComponentSpecs, IptComponentSpecsData};
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет связь компонента с разделами каталога.
pub(crate) fn del_component_specs(
    logged_user_uuid: &Uuid,
    data: &IptComponentSpecsData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &data.component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // creating structures for delete records
    let del_specs: DeleteComponentSpecs = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }

    diesel::delete(spec_to_component::spec_to_component)
        .filter(
            spec_to_component::component_uuid
                .eq(&del_specs.component_uuid)
                .and(spec_to_component::spec_id.eq_any(&del_specs.spec_ids)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
