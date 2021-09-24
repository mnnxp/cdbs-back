use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::modification_file_from_fileset::model::IptModificationFileFromFilesetData;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// The return the pre-signed URLs (in wrapper UploadFile) to upload the files
/// before that inserts rows in in file_ref and modification_file_from_fileset tables
pub(crate) fn add_files_of_modification_set(
    target_user_uuid: &Uuid,
    data: IptModificationFileFromFilesetData,
    conn: &PgConnection
) -> ServiceResult<Vec<UploadFile>> {
    // return error if not found correct filename
    if data.filename.is_empty() {
        return Err(ServiceError::BadRequest("Not found filename".to_string()))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filename {
        // insert row file in file_ref and addiction tables
        let slim_file = file::service::register::register(
            PreliminaryFileData::from_ipt_file_data( // <-- making data for insert
                *target_user_uuid,
                Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
                ListObject::ComponentModificationSet(data.fileset_uuid),
                filename,
                conn
            ),
            conn
        )?;

        debug!("New modification file: {:?}", slim_file);

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
