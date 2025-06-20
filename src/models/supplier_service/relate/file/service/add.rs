use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::models::supplier_service::access::util::check_access_service_for_user;
use crate::models::supplier_service::file::model::IptServiceFilesData;
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Generates preliminary file information for the service.
/// Returns structures with a pre-signed URL for file uploads.
pub(crate) fn add_service_files(
    data: &IptServiceFilesData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn,
    )?;

    // return error if not correct file name
    if data.filenames.is_empty() || data.filenames.len() > 100 {
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

        let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file)?;

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
