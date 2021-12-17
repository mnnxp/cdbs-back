use crate::errors::{ServiceResult, ServiceError};
use crate::storage::model::{StorageAccess, InsertablePresignedUrl};
use crate::storage::s3::Aws;
use chrono::{Duration, Local};
use diesel::prelude::*;
use uuid::Uuid;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };

    let presigned_url = Aws::from(access_storage).put_download_signed_url(
        &access_storage.bucket,
        path_file,
        opt.expiration_presigned_url,
    ).map_err(|err| {
        debug!("Failed make presign-url: {:#?}", err);
        ServiceError::InternalServerError
    })?;

    debug!("Presigned url for download: {:#?}", presigned_url);
    Ok(presigned_url)
}

/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(
    access_storage: &StorageAccess,
    path_file: &str,
) -> ServiceResult<String> {
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };

    let presigned_url = Aws::from(access_storage).get_upload_signed_url(
        &access_storage.bucket,
        path_file,
        opt.expiration_presigned_url,
    ).map_err(|err| {
        debug!("Failed make presign-url: {:#?}", err);
        ServiceError::InternalServerError
    })?;

    debug!("Presigned url for upload: {:#?}", presigned_url);
    Ok(presigned_url)
}

/// Save presign url for download to database
pub(crate) fn save_presign_url(
    file_uuid: &Uuid,
    presigned_url: &str,
    conn: &PgConnection
) -> ServiceResult<usize> {
    use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };

    // in db the url action time less than the real one
    let extension_time = match opt.expiration_presigned_url {
        800.. => opt.expiration_presigned_url - 400,
        _ => opt.expiration_presigned_url,
    };
    let naive_local_extension = Local::now().naive_local() + Duration::seconds(extension_time as i64);

    // save new presigned_url for download to db
    let update_url_to_db = diesel::update(presigned_url_ref::presigned_url_ref)
        .filter(presigned_url_ref::file_uuid.eq(file_uuid))
        .set((
            presigned_url_ref::presigned_url.eq(presigned_url),
            presigned_url_ref::expiration_at.eq(naive_local_extension)
        ))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update presigned_url {:?}", err);
            ServiceError::InternalServerError
        })?;

    match update_url_to_db {
        0 => {
            let insert_data = InsertablePresignedUrl{
                file_uuid: *file_uuid,
                presigned_url: presigned_url.to_string(),
                expiration_at: naive_local_extension,
            };

            diesel::insert_into(presigned_url_ref::presigned_url_ref)
                .values(&insert_data)
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed insert presigned_url {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        x => Ok(x),
    }
}
