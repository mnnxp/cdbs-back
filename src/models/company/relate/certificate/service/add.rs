use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::certificate::model::{
    CompanyCertificate, IptCompanyCertificateData, InsertableCompanyCertificate
};
use crate::models::company::access::util::check_company_access;
use crate::models::relate_ref::file as file;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use crate::schema::company_certificate_ref::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_certificate(
    logged_user_uuid: &Uuid,
    cert_data: &IptCompanyCertificateData,
    conn: &PgConnection,
) -> ServiceResult<UploadFile> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        logged_user_uuid,
        &cert_data.company_uuid,
        &need_access_level,
        conn
    )?;

    // Get data for write information about the file before upload to storage
    let preliminary_file_data = PreliminaryFileData::from_ipt_file_data(
        *logged_user_uuid,
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
        ListObject::CompanyCertificate(cert_data.company_uuid),
        &cert_data.filename,
        conn
    );

    let slim_file = file::service::register::register(
        preliminary_file_data,
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
        &StorageAccess::get(conn)?,
        &slim_file.path_file,
    )?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
