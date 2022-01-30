use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::fileset_for_program::file::model::IptModificationFileFromFilesetData;
use crate::models::component::component_modification::relate::fileset_for_program::util::get_component_by_fileset;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// The return the pre-signed URLs (in wrapper UploadFile) to upload the files
/// before that inserts rows in in file_ref and modification_file_from_fileset tables
pub(crate) fn add_files_of_modification_set(
    logged_user_uuid: &Uuid,
    data: &IptModificationFileFromFilesetData,
    conn: &PgConnection
) -> ServiceResult<Vec<UploadFile>> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&data.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // return error if not found correct filename
    if data.filenames.is_empty() || data.filenames.len() > 100 {
        return Err(ServiceError::BadRequest("Not found filename".to_string()))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filenames {
        // insert row file in file_ref and addiction tables
        let slim_file = file::service::register::preregister_file(
            PreliminaryFileData::from_ipt_file_data( // <-- making data for insert
                *logged_user_uuid,
                Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
                ListObject::ComponentModificationSet(data.fileset_uuid),
                filename,
                conn
            ),
            conn
        )?;

        debug!("New modification file: {:?}", slim_file);

        let upload_url = upload_presigned_url(
            &StorageAccess::from_env(),
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
