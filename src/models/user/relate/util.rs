use crate::errors::{ServiceResult, ServiceError};
use crate::schema::component_ref::dsl as component_ref;
use crate::schema::standard_ref::dsl as standard_ref;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Count standards for user by uuid
pub(crate) fn count_standards_for_user(
    target_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    standard_ref::standard_ref
        .filter(standard_ref::user_uuid.eq(target_user_uuid))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed count standards for user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Count components for user by uuid
pub(crate) fn count_components_for_user(
    target_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    component_ref::component_ref
        .filter(component_ref::user_uuid.eq(target_user_uuid))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed count components for user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Count companies for user by uuid
pub(crate) fn count_companies_for_user(
    target_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    company_ref::company_ref
        .filter(company_ref::user_uuid.eq(target_user_uuid))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed count companies for user: {:?}", err);
            ServiceError::InternalServerError
        })
}
