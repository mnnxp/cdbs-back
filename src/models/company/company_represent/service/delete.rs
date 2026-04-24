use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет информацию о представительстве компании.
pub(crate) fn delete_company_represent(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    target_represent_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_represent_ref::dsl::*;

    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        target_company_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // debug!("fn target_company_uuid = {}", &target_company_uuid);
    // debug!("fn target_represent_uuid = {}", &target_represent_uuid);

    // find represent and check privileges for delete
    let target_represent_uuid = company_represent_ref
        .filter(
            company_uuid
                .eq(target_company_uuid)
                .and(uuid.eq(target_represent_uuid)),
        )
        .select(uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed search represent: {:?}", err);
            get_err_msg(ErrorMessage::NotFoundRepresentative)
        })?;

    let res = diesel::delete(company_represent_ref.filter(uuid.eq(&target_represent_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete represent: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(res > 0)
}
