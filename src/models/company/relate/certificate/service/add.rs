use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::certificate::model::{
    CompanyCertificate, InsertableCompanyCertificate, IptCompanyCertificateData,
};
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::schema::company_certificate_ref::dsl::*;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

/// Загрузка нового сертификата компании.
/// Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла (сертификата).
pub(crate) fn add_certificate(
    logged_user_uuid: &Uuid,
    cert_data: &IptCompanyCertificateData,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    // update data validation
    if cert_data.description.chars().count() > 500 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(500)));
    }

    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &cert_data.company_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::CompanyCertificate(cert_data.company_uuid),
        &cert_data.filename,
        &Commit::create_commit("Upload certificate for company", conn)?,
        conn,
    )?;

    // workaround until i figure make the pre-url generation
    // let temp_string = format!("This will be url for upload file {:?}", slim_file.path_file);

    let new_company_certificate = InsertableCompanyCertificate {
        file_uuid: slim_file.uuid,
        company_uuid: cert_data.company_uuid,
        description: cert_data.description.clone(),
    };

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    let company_inserted_certificate: CompanyCertificate =
        diesel::insert_into(company_certificate_ref)
            .values(new_company_certificate)
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed insert new_company_certificate: {:?} ", err);
                ServiceError::InternalServerError
            })?;

    debug!(
        "Company inserted certificate: {:?}",
        company_inserted_certificate
    );

    let upload_url =
        upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file, domain)?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
