use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::file::model::{
    SlimFile,
    DownloadFile,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::download_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn get_url_file_by_uuid(
    _logged_user_uuid: &Uuid, // <-- todo!(access check)
    target_file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let storage_access = StorageAccess::get(conn)?;

    let show_file = SlimFile::get_file_by_uuid(
        target_file_uuid,
        conn,
    ).unwrap();

    download_presigned_url(
        &storage_access,
        &show_file.path_file,
    )
}

/// Gets presigned urls for target files by uuids
pub(crate) fn get_urls_files_by_uuid(
    target_file_uuids: &[Uuid],
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    // return error if not found files
    if target_file_uuids.is_empty() {
        return Err(ServiceError::BadRequest("Not found files".to_string()))
    }

    let storage_access = StorageAccess::get(conn)?;

    // get files data by uuids, return error if have fail
    let show_files = match SlimFile::get_file_by_vec_uuid(
        target_file_uuids,
        conn,
    ) {
        Ok(data) => data,
        Err(err) => {
            debug!("Fail get presigned url: {:?}", err);
            return Err(ServiceError::BadRequest("Fail get files data".to_string()))
        },
    };

    let mut res_down_urls: Vec<DownloadFile> = Vec::new();
    for sf in show_files {
        match download_presigned_url(
            &storage_access,
            &sf.path_file,
        ) {
            Ok(sig_url) => res_down_urls.push(DownloadFile {
                uuid: sf.uuid,
                filename: sf.filename,
                filesize: sf.filesize,
                download_url: sig_url,
            }),
            Err(err) => {
                debug!("Fail get presigned url: {:?}", err);
                res_down_urls.push(DownloadFile {
                    uuid: sf.uuid,
                    filename: sf.filename,
                    filesize: sf.filesize,
                    download_url: "Failed get url".to_string(),
                })
            },
        }
    }

    debug!("Gets presigned urls: {:?}", res_down_urls);

    Ok(res_down_urls)
}
