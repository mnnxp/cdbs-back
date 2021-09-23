use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::relate::file::model::IptComponentFileData;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// The return the pre-signed URLs (in wrapper UploadFile) to download the file
/// and insert the line to link the file to the component
pub(crate) fn add_component_files(
    target_user_uuid: &Uuid,
    data: &IptComponentFileData,
    conn: &PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    // return error if not found correct keywords
    if data.filename.is_empty() {
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filename {
        let slim_file = file::service::register::register(
            PreliminaryFileData::from_ipt_file_data(
                *target_user_uuid,
                Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
                ListObject::Component(data.component_uuid),
                filename,
                conn
            ),
            conn
        )?;

        debug!("New component file: {:?}", slim_file);

        up_files.push(UploadFile{
            filename: filename.to_string(),
            upload_url: upload_presigned_url(
                &StorageAccess::get(conn)?,
                &slim_file.path_file,
            )?,
        });
    }

    Ok(up_files)
}
