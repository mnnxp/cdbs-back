use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::SlimFile;
use crate::storage::model::{InsertablePresignedUrl, StorageAccess};
use crate::storage::s3::Aws;
use chrono::{Duration, Local};
use diesel::prelude::*;
use uuid::Uuid;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(
    access_storage: &StorageAccess,
    slim_file: &SlimFile,
) -> ServiceResult<String> {
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };

    let presigned_url = Aws::from(access_storage)
        .download_presigned_url(
            &access_storage.bucket,
            slim_file,
            opt.s3_expiration_presigned_url,
        )
        .map_err(|err| {
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

    Aws::from(access_storage)
        .get_upload_signed_url(
            &access_storage.bucket,
            path_file,
            opt.s3_expiration_presigned_url,
        )
        .map_err(|err| {
            debug!("Failed make presign-url: {:#?}", err);
            ServiceError::InternalServerError
        })
}

/// Save presign url for download to database
pub(crate) fn save_presign_url(
    file_uuid: &Uuid,
    presigned_url: &str,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
    let opt = {
        use structopt::StructOpt;
        crate::cli_args::Opt::from_args()
    };

    // in db the url action time less than the real one
    let expiration_at = match opt.s3_expiration_presigned_url {
        800.. => opt.s3_expiration_presigned_url - 400,
        _ => opt.s3_expiration_presigned_url,
    };
    let new_expiration_at = Local::now().naive_local() + Duration::seconds(expiration_at as i64);

    // save new presigned_url for download to db
    let check_old_url = presigned_url_ref::presigned_url_ref
        .filter(presigned_url_ref::file_uuid.eq(file_uuid))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed check old presigned_url {:?}", err);
            ServiceError::InternalServerError
        })?;

    match check_old_url {
        0 => {
            let insert_data = InsertablePresignedUrl {
                file_uuid: *file_uuid,
                presigned_url: presigned_url.to_string(),
                expiration_at: new_expiration_at,
            };

            diesel::insert_into(presigned_url_ref::presigned_url_ref)
                .values(&insert_data)
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed insert presigned_url {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        x => {
            debug!("Found old presigned_url: {:?}", x);
            diesel::update(presigned_url_ref::presigned_url_ref)
                .filter(presigned_url_ref::file_uuid.eq(file_uuid))
                .set((
                    presigned_url_ref::presigned_url.eq(presigned_url),
                    presigned_url_ref::expiration_at.eq(new_expiration_at),
                ))
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed update presigned_url {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}
