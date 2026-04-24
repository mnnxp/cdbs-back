use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
    util::check_image_filename,
};
use crate::models::standard::file::model::{IptStandardFaviconData, IptStandardFilesData};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

// Генерирует предварительную информацию о файлах для стандарта.
/// Возвращает структуры с предварительно подписанным URL-адресом для загрузки файлов.
pub(crate) fn add_standard_files(
    logged_user_uuid: &Uuid,
    data: &IptStandardFilesData,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        &data.standard_uuid,
        AccessOperation::Manage,
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
            ListObject::Standard(data.standard_uuid),
            filename,
            &commit_uuid,
            conn,
        )?;

        debug!("New standard file: {:?}", slim_file);

        let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file, domain)?;

        up_files.push(UploadFile {
            file_uuid: slim_file.uuid,
            filename: slim_file.filename,
            upload_url,
        });
    }

    Ok(up_files)
}

/// Обновляет основное изображение стандарта.
/// Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла изображения.
pub(crate) fn add_standard_favicon(
    logged_user_uuid: &Uuid,
    data: &IptStandardFaviconData,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        &data.standard_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // return error if not correct file name
    if data.filename.is_empty() || data.filename.len() > 500 {
        return Err(get_err_msg(ErrorMessage::BadFilename));
    }

    // return error if not correct file name
    if !check_image_filename(&data.filename) {
        return Err(get_err_msg(ErrorMessage::SelectedFileIsNotImage));
    }

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::StandardFavicon(data.standard_uuid),
        &data.filename,
        &Commit::create_commit("Upload main image of the standard", conn)?,
        conn,
    )?;

    debug!("New standard favicon: {:?}", slim_file);

    let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file, domain)?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
