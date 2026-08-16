use crate::config::{aws_client, s3_bucket, s3_expiration_presigned_url};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::SlimFile;
use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
use crate::storage::model::InsertablePresignedUrl;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use log::debug;
use uuid::Uuid;

use super::model::S3Proxer;

/// Gets presigned url for target file by path
pub(crate) fn download_presigned_url(slim_file: &SlimFile) -> ServiceResult<String> {
    let presigned_url = aws_client()
        .get_download_presigned_url(s3_bucket(), slim_file, s3_expiration_presigned_url())
        .map_err(|err| {
            debug!("Failed make presign-url: {:#?}", err);
            ServiceError::InternalServerError
        })?;

    debug!("Presigned url for download: {:#?}", presigned_url);
    Ok(presigned_url)
}

/// Gets presigned url for upload files to storage
pub(crate) fn upload_presigned_url(path_file: &str, domain: &str) -> ServiceResult<String> {
    aws_client()
        .get_upload_signed_url(s3_bucket(), path_file, s3_expiration_presigned_url())
        .map(|url| url.proxied(domain))
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
    let expiration = s3_expiration_presigned_url();

    // in db the url action time less than the real one
    let expiration_at = match expiration {
        800.. => expiration - 400,
        _ => expiration,
    };
    let new_expiration_at = Utc::now().naive_utc() + Duration::seconds(expiration_at as i64);

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
