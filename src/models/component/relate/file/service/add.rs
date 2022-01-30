use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::relate::file::model::{
    IptComponentFilesData, IptComponentFaviconData
};
use crate::models::relate_ref::file::{
    model::{ListObject, PreliminaryFileData, UploadFile},
    service::register::preregister_file,
    util::{check_image_filename, get_default_image}
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

/// The return the pre-signed URLs (in wrapper UploadFile) to upload the file to storage
/// and insert the line to link the file to the component
pub(crate) fn add_component_files(
    logged_user_uuid: &Uuid,
    data: &IptComponentFilesData,
    conn: &PgConnection,
) -> ServiceResult<Vec<UploadFile>> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // return error if not correct file name
    if data.filenames.is_empty() || data.filenames.len() > 100 {
        return Err(ServiceError::BadRequest("Bad filename".to_string()))
    }

    let mut up_files: Vec<UploadFile> = Vec::new();
    // Get data for write information about the file before upload to storage
    for filename in &data.filenames {
        let slim_file = preregister_file(
            PreliminaryFileData::from_ipt_file_data(
                *logged_user_uuid,
                get_default_image(),
                ListObject::Component(data.component_uuid),
                filename,
                conn
            ),
            conn
        )?;

        debug!("New component file: {:?}", slim_file);

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

/// The return the pre-signed URLs (in wrapper UploadFile) to upload the file to storage
/// and insert the line to link the file to the component
pub(crate) fn add_component_favicon(
    logged_user_uuid: &Uuid,
    data: &IptComponentFaviconData,
    conn: &PgConnection,
) -> ServiceResult<UploadFile> {

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // return error if not correct file name
    if data.filename.is_empty() || data.filename.len() > 100 {
        return Err(ServiceError::BadRequest("Bad filename".to_string()))
    }

    // return error if not correct file name
    if !check_image_filename(&data.filename) {
        return Err(ServiceError::BadRequest("Selected file is not image.".to_string()))
    }

    let slim_file = preregister_file(
        PreliminaryFileData::from_ipt_file_data(
            *logged_user_uuid,
            get_default_image(),
            ListObject::ComponentFavicon(data.component_uuid),
            &data.filename,
            conn
        ),
        conn
    )?;

    debug!("New component file: {:?}", slim_file);

    let upload_url = upload_presigned_url(
        &StorageAccess::from_env(),
        &slim_file.path_file,
    )?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
