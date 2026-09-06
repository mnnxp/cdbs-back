use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::models::supplier_service::file::model::IptServiceFilesData;
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Generates preliminary file information for the service.
/// Returns structures with a pre-signed URL for file uploads.
pub(crate) fn add_service_files(
    data: &IptServiceFilesData,
    logged_user_uuid: &Uuid,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Service,
        &data.service_uuid,
        AccessOperation::Write,
        conn,
    )?;

    // return error if files not found or more than 500 files in one request
    if data.filenames.is_empty() || data.filenames.len() > 500 {
        return Err(get_err_msg(ErrorMessage::BadFilename));
    }

    // create commit message for the changes
    let commit_uuid = Commit::create_commit(&data.commit_msg, conn)?;
    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filenames {
        let slim_file = preregister_file(
            logged_user_uuid,
            ListObject::Service(data.service_uuid),
            filename,
            &commit_uuid,
            conn,
        )?;

        debug!("New service file: {:?}", slim_file);

        let upload_url = upload_presigned_url(&slim_file.path_file, domain)?;

        up_files.push(UploadFile {
            file_uuid: slim_file.uuid,
            filename: slim_file.filename,
            upload_url,
        });
    }
    if !up_files.is_empty() {
        change_service_updated_at(
            &data.service_uuid,
            logged_user_uuid,
            format!("File(s) prepared for uploading: {:?}", &data.filenames),
            conn,
        )?;
    }
    Ok(up_files)
}
