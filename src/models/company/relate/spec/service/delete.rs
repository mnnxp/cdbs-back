use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::spec::model::{DelCompanySpec, IptCompanySpecsData};
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет подключения компании к указанным разделам каталога.
pub(crate) fn del_company_specs(
    logged_user_uuid: &Uuid,
    data: &IptCompanySpecsData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    use crate::schema::spec_to_company::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &data.company_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // creating structures for delete records
    let del_specs: DelCompanySpec = data.into();

    if del_specs.spec_ids.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }

    diesel::delete(spec_to_company)
        .filter(
            company_uuid
                .eq(&del_specs.company_uuid)
                .and(spec_id.eq_any(&del_specs.spec_ids)),
        )
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })
}
