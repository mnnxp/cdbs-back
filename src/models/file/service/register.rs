use std::fs;
use std::io::Write;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::file::model::{
    InsertableFile, SlimFile, File, FileData,
    InsertableFileToComponent, InsertableFileToModification, FileToModel
};
use crate::models::file as file;

use diesel::prelude::*;
use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

// todo!(move this out in a .env file)
const UPLOAD_PATH: &str = "/home/mnnxp/Downloads/tmp/upload";

pub(crate) async fn register(
    payload: Multipart,
    user_uuid: Uuid,
    addiction_table: u8,
    uuid_addiction: Uuid,
    uuid_file_parent: Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<SlimFile>> {
    // Check uuid correct and bound with active user
    let owned_correct: i32 = match addiction_table {
        1_u8 => 1,                                                                            // <-- addiction table not need
        2_u8 => file::util::check_user_owned_component(user_uuid, uuid_addiction, conn),      // <-- select addiction table: file_to_component
        3_u8 => file::util::check_user_owned_modification(user_uuid, uuid_addiction, conn),   // <-- select addiction table: file_to_modification
        _ => ServiceResult::Err(ServiceError::BadRequest("Bad...".to_string()))?
    };

    match owned_correct {
        1..=i32::MAX => write_file(
                payload, user_uuid, uuid_file_parent, addiction_table, uuid_addiction, conn
            ).await,
        _ => ServiceResult::Err(ServiceError::BadRequest("Not found data for this uuid.".to_string()))?
    }
}

pub(crate) async fn write_file(
    mut payload: Multipart,
    user_uuid: Uuid,
    uuid_file_parent: Uuid,
    addiction_table: u8,
    uuid_addiction: Uuid,
    conn: &PgConnection
) -> ServiceResult<Vec<SlimFile>> {

    // iterate over multipart stream
    fs::create_dir_all(UPLOAD_PATH).unwrap();

    let mut slim_file_data: Vec<SlimFile> = Vec::new();
    // let mut addiction_data: Vec<FileToModel> = Vec::new();

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

            let file_metadata = file::util::metadata(&out_filepath, &filename, conn);

            // debug!("Hash TEST FILE {:?}", &file_metadata.hash);
            // debug!("uuid_file_parent before: {:#?}", uuid_file_parent);

            let file_metadata = FileData {
                path_file: (out_filepath),
                filesize: (file_metadata.filesize),
                id_ext: (file_metadata.id_ext),
                filename: (filename),
                uuid_user: (user_uuid),
                hash: (file_metadata.hash),
                uuid_file_parent: (uuid_file_parent),
            };

            // debug!("uuid_file_parent after: {:#?}", uuid_file_parent);

            // register data on db file_ref and addiction table (depends on the request)
            let value_slim_file_data = write_metadata(file_metadata, conn)?;

            match addiction_table {
                1_u8 => (),
                2_u8..=4_u8 => {
                    // colloborate data for addiction table
                    let addiction_data = FileToModel {
                        uuid_file: value_slim_file_data.uuid,
                        uuid: uuid_addiction,
                    };
                    write_addiction_data(addiction_data, addiction_table, conn)?;
                },
                _ => ServiceResult::Err(ServiceError::BadRequest("Error write data: addiction not found.".to_string()))?
            };

            // add data in response for user
            slim_file_data.push(value_slim_file_data)
        }
    // debug!("Slim_file_data Vec: {:#?}", &slim_file_data);

    // if response is empty - this error
    if slim_file_data.is_empty() {
        ServiceResult::Err(ServiceError::BadRequest("Data not found.".to_string()))?
    }

    Ok(slim_file_data)
}


/// Write information of file to db file_ref
pub(crate) fn write_metadata(
    file_data: FileData,
    conn: &PgConnection
) -> ServiceResult<SlimFile> {
    use crate::schema::file_ref::dsl::file_ref;

    let file: InsertableFile = file_data.into();
    let inserted_file: File = diesel::insert_into(file_ref).values(&file).get_result(conn)?;
    Ok(inserted_file.into())
}

/// Write information of file to db file_to_component or file_to_modification
pub(crate) fn write_addiction_data(
    addiction_data: FileToModel,
    addiction_table: u8,
    conn: &PgConnection
) -> ServiceResult<FileToModel>{
    // select addiction table for write additional data
    match addiction_table {
        2_u8 => {   // <- add addiction data in file_to_component
            use crate::schema::file_to_component::dsl::file_to_component;

            let addiction: InsertableFileToComponent = addiction_data.into();
            let inserted_addiction: FileToModel = diesel::insert_into(file_to_component)
                .values(&addiction)
                .get_result(conn)?;

            debug!("Select addiction table: file_to_component, data: {:?} ", &inserted_addiction);

            Ok(inserted_addiction)
        },
        3_u8 => {   // <- add addiction data in file_to_modification
            use crate::schema::file_to_modification::dsl::file_to_modification;

            let addiction: InsertableFileToModification = addiction_data.into();
            let inserted_addiction: FileToModel = diesel::insert_into(file_to_modification)
                .values(&addiction)
                .get_result(conn)?;

            debug!("Select addiction table: file_to_modification, data: {:?} ", &inserted_addiction);

            Ok(inserted_addiction)
        },
        _ => ServiceResult::Err(ServiceError::BadRequest("Error select addiction_table".to_string()))?
    }
}
