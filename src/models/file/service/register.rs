use std::fs;
use std::io::Write;
use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::file::model::{InsertableFile, SlimFile, File, FileData};
// use crate::models::file::service::metadata::ParseFileData;
use crate::models::file::service as file;

use diesel::prelude::*;
use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

const UPLOAD_PATH: &str = "/home/mnnxp/Downloads/tmp/upload";

pub(crate) async fn register(
    payload: Multipart,
    user_uuid: Uuid,
    // uuid_file_parent: Uuid,
    pool: web::Data<Pool>
) -> ServiceResult<Vec<SlimFile>> {
    let conn = &db_connection(&pool)?;

    // TODO: add search for parent file by name in table file_ref
    let uuid_file_parent = Uuid::parse_str("3706d1a1-80ae-4367-be39-af7091373811")?;

    write_file(payload, user_uuid, uuid_file_parent, conn).await
}

// pub(crate) async fn create_file(
//     payload: Multipart,
//     user_uuid: Uuid,
//     uuid_file_parent: Uuid,
//     conn: &PgConnection
// ) -> ServiceResult<Vec<SlimFile>> {
//     write_file(payload, user_uuid, uuid_file_parent, conn).await
// }

pub(crate) async fn write_file(
    mut payload: Multipart,
    user_uuid: Uuid,
    uuid_file_parent: Uuid,
    conn: &PgConnection
) -> ServiceResult<Vec<SlimFile>> {

    // iterate over multipart stream
    fs::create_dir_all(UPLOAD_PATH).unwrap();

    let mut slim_file_data: Vec<SlimFile> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let filename = sanitize_filename::sanitize(&content_type.get_filename().unwrap());

            let filepath = format!("{}/{}-{}",
                UPLOAD_PATH,
                user_uuid,
                Uuid::new_v4()
            );

            let out_filepath = filepath.clone();

            // File::create is blocking operation, use thread pool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use thread pool
                f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
            }

            let file_metadata = file::metadata(&out_filepath, &filename, conn);

            // debug!("Hash TEST FILE {:?}", &file_metadata.hash);
            // debug!("uuid_file_parent before: {:#?}", uuid_file_parent);

            let file_metadata = FileData {
                path_file: (out_filepath),
                filesize: (file_metadata.filesize),
                id_ext: (file_metadata.id_ext),
                filename: (filename),
                uuid_user_create: (user_uuid),
                hash: (file_metadata.hash),
                uuid_file_parent: (uuid_file_parent),
            };

            // debug!("uuid_file_parent after: {:#?}", uuid_file_parent);

            // add data in response for user
            let value_slim_file_data = write_metadata(file_metadata, conn)?;

            slim_file_data.push(value_slim_file_data)
        }
    // debug!("Slim_file_data Vec: {:#?}", &slim_file_data);

    // if response is empty - this error
    if slim_file_data.is_empty() {
        ServiceResult::Err(ServiceError::BadRequest("Data not found. You okay?".to_string()))?
    }

    Ok(slim_file_data)
}

pub(crate) fn write_metadata(file_data: FileData, conn: &PgConnection) -> ServiceResult<SlimFile> {
    use crate::schema::file_ref::dsl::file_ref;

    let file: InsertableFile = file_data.into();
    let inserted_file: File = diesel::insert_into(file_ref).values(&file).get_result(conn)?;
    Ok(inserted_file.into())
}
