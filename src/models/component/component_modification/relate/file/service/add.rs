use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::component_modification::relate::file::model::IptModificationFilesData;
use crate::models::component::component_modification::util::get_component_by_modification;
use crate::models::relate_ref::file::{
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::models::component::access::util::check_access_component_for_user;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Создает предварительную информацию в файлах для модификации компонента.
/// Возвращает структуры с предварительно подписанным URL-адресом для загрузки файлов.
pub(crate) fn add_modification_files(
    logged_user_uuid: &Uuid,
    data: &IptModificationFilesData,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&data.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // return error if not found correct filename
    if data.filenames.is_empty() || data.filenames.len() > 100 {
        return Err(get_err_msg(ErrorMessage::NotFoundFilename))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filenames {
        // insert row file in file_ref and addiction tables
        let slim_file = preregister_file(
            logged_user_uuid,
            ListObject::ComponentModification(data.modification_uuid),
            filename,
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
