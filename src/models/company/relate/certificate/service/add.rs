use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::certificate::model::{
    CompanyCertificate, IptCompanyCertificateData, InsertableCompanyCertificate
};
use crate::models::company::access::util::check_company_access;
use crate::models::relate_ref::file::{
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use crate::schema::company_certificate_ref::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Загрузка нового сертификата компании.
/// Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла (сертификата).
pub(crate) fn add_certificate(
    logged_user_uuid: &Uuid,
    cert_data: &IptCompanyCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        logged_user_uuid,
        &cert_data.company_uuid,
        &need_access_level,
        conn
    )?;

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::CompanyCertificate(cert_data.company_uuid),
        &cert_data.filename,
        conn
    )?;

    // workaround until i figure make the pre-url generation
    // let temp_string = format!("This will be url for upload file {:?}", slim_file.path_file);

    let new_company_certificate = InsertableCompanyCertificate{
        file_uuid: slim_file.uuid,
        company_uuid: cert_data.company_uuid,
        description: cert_data.description.clone(),
    };

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    let company_inserted_certificate: CompanyCertificate = diesel::insert_into(company_certificate_ref)
        .values(new_company_certificate)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed insert new_company_certificate: {:?} ", err);
            ServiceError::InternalServerError
        })?;

    debug!("Company inserted certificate: {:?}", company_inserted_certificate);

    let upload_url = upload_presigned_url(
        &StorageAccess::from_env(),
        &slim_file.path_file,
    )?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
