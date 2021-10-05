use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::file::model::IptStandardFilesData;
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// The return the pre-signed URLs (in wrapper UploadFile) to download the file
/// and insert the line to link the file to the standard
pub(crate) fn add_standard_files(
    logged_user_uuid: &Uuid,
    data: &IptStandardFilesData,
    conn: &PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn,
    )?;

    // return error if not correct file name
    if data.filename.is_empty() {
        return Err(ServiceError::BadRequest("Bad filename".to_string()))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filename {
        let slim_file = file::service::register::register(
            PreliminaryFileData::from_ipt_file_data(
                *logged_user_uuid,
                Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
                ListObject::Standard(data.standard_uuid),
                filename,
                conn
            ),
            conn
        )?;

        debug!("New standard file: {:?}", slim_file);

        let upload_url = upload_presigned_url(
            &StorageAccess::get(conn)?,
            &slim_file.path_file,
        )?;

        up_files.push(UploadFile {
            file_uuid: slim_file.uuid,
            filename: slim_file.filename,
            upload_url,
        });
    }

    Ok(up_files)
}
