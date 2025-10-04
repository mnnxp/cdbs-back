use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::company::relate::certificate::service::delete::delete_company_certificates;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет компанию и связанные с ней данные. Возвращает идентификатор удаленной компании.
pub(crate) fn del_company(
    logged_user_uuid: &Uuid,
    del_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // check user owner company
    check_is_owner_with_err(logged_user_uuid, del_company_uuid, conn)?;

    delete_company(del_company_uuid, conn)
}

/// Delete all company and related data
pub(crate) fn delete_company(
    del_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // delete company certificates and set flags for certificates files
    delete_company_certificates(del_company_uuid, conn)?;

    let image_file_uuid = diesel::delete(
        company_ref::company_ref
            // .filter(company_ref::user_uuid.eq(logged_user_uuid) // <-- only companies the user
            .filter(company_ref::uuid.eq(del_company_uuid)),
    )
    .returning(company_ref::image_file_uuid)
    .get_result::<Uuid>(conn)
    .map_err(|err| {
        debug!("Failed delete company: {:?}", err);
        ServiceError::InternalServerError
    })?;

    // set delete flag for main image deleted a company
    delete_file_by_uuid(&image_file_uuid, conn)?;

    Ok(*del_company_uuid)
}
