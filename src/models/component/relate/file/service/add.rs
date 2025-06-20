use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::relate::file::model::{
    IptComponentFaviconData, IptComponentFilesData,
};
use crate::models::component::service::update::change_updated_at;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
    util::check_image_filename,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Генерирует предварительную информацию о файлах для компонента.
/// Возвращает структуры с предварительно подписанным URL-адресом для загрузки файлов.
pub(crate) fn add_component_files(
    logged_user_uuid: &Uuid,
    data: &IptComponentFilesData,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
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
            ListObject::Component(data.component_uuid),
            filename,
            &commit_uuid,
            conn,
        )?;

        debug!("New component file: {:?}", slim_file);

        let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file)?;

        up_files.push(UploadFile {
            file_uuid: slim_file.uuid,
            filename: slim_file.filename,
            upload_url,
        });
    }
    // update the updated_at date if new files are added
    if !up_files.is_empty() {
        change_updated_at(&data.component_uuid, None, conn)?;
    }

    Ok(up_files)
}

/// Обновляет основное изображение компонента.
/// Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла изображения.
pub(crate) fn add_component_favicon(
    logged_user_uuid: &Uuid,
    data: &IptComponentFaviconData,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn,
    )?;

    // return error if not correct file name
    if data.filename.is_empty() || data.filename.len() > 100 {
        return Err(get_err_msg(ErrorMessage::BadFilename));
    }

    // return error if not correct file name
    if !check_image_filename(&data.filename) {
        return Err(get_err_msg(ErrorMessage::SelectedFileIsNotImage));
    }

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::ComponentFavicon(data.component_uuid),
        &data.filename,
        &Commit::create_commit("Upload main image of the component", conn)?,
        conn,
    )?;

    debug!("New component file: {:?}", slim_file);

    let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file)?;

    change_updated_at(&data.component_uuid, None, conn)?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
