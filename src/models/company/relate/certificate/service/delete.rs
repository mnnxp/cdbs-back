use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::certificate::model::DelCompanyCertificateData;
use crate::models::company::access::util::check_company_access;
use crate::models::relate_ref::file::service::delete::{
    delete_file_by_uuid, delete_file_by_uuids
};
use crate::schema::company_certificate_ref::dsl as company_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет сертификат компании.
pub(crate) fn del_certificate_description(
    logged_user_uuid: &Uuid,
    data: &DelCompanyCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    // delete row company certificate
    let file_uuid = diesel::delete(company_certificate_ref::company_certificate_ref
        .filter(company_certificate_ref::company_uuid.eq(&data.company_uuid)
        .and(company_certificate_ref::file_uuid.eq(&data.file_uuid))))
        .returning(company_certificate_ref::file_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed remove certificate data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // delete file rows and file in storage
    delete_file_by_uuid(&file_uuid, conn)
}

/// Set the delete flags for all company certificates
pub(crate) fn delete_company_certificates(
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let del_file_uuids = diesel::delete(company_certificate_ref::company_certificate_ref
        .filter(company_certificate_ref::company_uuid.eq(company_uuid)))
        .returning(company_certificate_ref::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
