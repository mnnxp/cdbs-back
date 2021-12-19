use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::certificate::model::{
    UserCertificate, IptUserCertificateData, InsertableUserCertificate
};
use crate::models::relate_ref::file as file;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::schema::user_certificate_ref::dsl::*;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_certificate(
    logged_user_uuid: &Uuid,
    cert_data: &IptUserCertificateData,
    conn: &PgConnection,
) -> ServiceResult<UploadFile> {
    // Get data for write information about the file before upload to storage
    let preliminary_file_data = PreliminaryFileData::from_ipt_file_data(
        *logged_user_uuid,
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
        ListObject::UserCertificate(*logged_user_uuid),
        &cert_data.filename,
        conn
    );

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    // workaround until i figure make the pre-url generation
    // let temp_string = format!("This will be url for upload file {:?}", slim_file.path_file);

    let new_user_certificate = InsertableUserCertificate{
        file_uuid: slim_file.uuid,
        user_uuid: *logged_user_uuid,
        description: cert_data.description.to_string(),
    };

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    let user_inserted_certificate = diesel::insert_into(user_certificate_ref)
        .values(new_user_certificate)
        .get_result::<UserCertificate>(conn)
        .map_err(|err| {
            debug!("Failed insert certificate data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    debug!("User inserted certificate: {:?}", user_inserted_certificate);

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
