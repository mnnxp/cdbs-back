use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::fileset_for_program::file::model::IptModificationFileFromFilesetData;
use crate::models::component::component_modification::fileset_for_program::util::get_modification_by_fileset;
use crate::models::component::component_modification::relate::fileset_for_program::util::get_component_by_fileset;
use crate::models::component::service::update::change_updated_at;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// Создает предварительную информацию о файлах для набора файлов из модификации компонента.
/// Возвращает структуры с предварительно подписанным URL-адресом для загрузки файлов.
pub(crate) fn add_files_of_modification_set(
    logged_user_uuid: &Uuid,
    data: &IptModificationFileFromFilesetData,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<UploadFile>> {
    let need_access_level = 1; // todo!(create enum for manage access level)
    let target_component_uuid = get_component_by_fileset(&data.fileset_uuid, conn)?;
    let target_modification_uuid = get_modification_by_fileset(&data.fileset_uuid, conn)?;
    check_access_component_for_user(
        logged_user_uuid,
        &target_component_uuid,
        need_access_level,
        conn,
    )?;

    // return error if files not found or more than 500 files in one request
    if data.filenames.is_empty() || data.filenames.len() > 500 {
        return Err(get_err_msg(ErrorMessage::NotFoundFilename));
    }

    // create commit message for the changes
    let commit_uuid = Commit::create_commit(&data.commit_msg, conn)?;
    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filenames {
        // insert row file in file_ref and addiction tables
        let slim_file = preregister_file(
            logged_user_uuid,
            ListObject::ComponentModificationSet(data.fileset_uuid),
            filename,
            &commit_uuid,
            conn,
        )?;

        debug!("New modification file: {:?}", slim_file);

        let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file, domain)?;

        up_files.push(UploadFile {
            file_uuid: slim_file.uuid,
            filename: slim_file.filename,
            upload_url,
        });
    }
    // update the updated_at for component and modification if new files are added
    if !up_files.is_empty() {
        change_updated_at(
            &target_component_uuid,
            Some(&target_modification_uuid),
            conn,
        )?;
    }
    Ok(up_files)
}
