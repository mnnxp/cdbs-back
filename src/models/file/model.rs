// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
// use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;
// use crate::models::file::util::hex_to_bytes;
// type NaiveDateTime = chrono::NaiveDateTime;
// Main file structures

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_file_parent: Uuid,
    pub hash: Vec<u8>,
    pub uuid_user: Uuid,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: i32,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowFile {
    pub uuid: Uuid,
    pub uuid_file_parent: Uuid,
    pub uuid_user: Uuid,
    pub filename: String,
    pub id_ext: i32,
    pub value_ext: String,
    pub filesize: i32,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "file_ref"]
pub struct InsertableFile {
    pub uuid: Uuid,
    pub uuid_file_parent: Uuid,
    pub hash: Vec<u8>,
    pub uuid_user: Uuid,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: i32,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub uuid_file_parent: Uuid,
    pub hash:  Vec<u8>,
    pub uuid_user: Uuid,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: i32,
    pub path_file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimFile {
    pub uuid: Uuid,
    pub filename: String,
    pub filesize: i32,
    pub path_file: String,
}

// Related file structures
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct FileToModel {
    pub id: i32,
    pub uuid_file: Uuid,
    pub uuid: Uuid,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct FileToModelData {
    pub uuid_file: Uuid,
    pub uuid: Uuid,
}

// Structures for Component
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct FileToComponent {
    pub id: i32,
    pub uuid_file: Uuid,
    pub uuid_component: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_component"]
pub struct InsertableFileToComponent {
    pub uuid_file: Uuid,
    pub uuid_component: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct FileToModification {
    pub id: i32,
    pub uuid_file: Uuid,
    pub uuid_modification: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_modification"]
pub struct InsertableFileToModification {
    pub uuid_file: Uuid,
    pub uuid_modification: Uuid,
}

// Structures for Standard
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct FileToStandard {
    pub id: i32,
    pub uuid_file: Uuid,
    pub uuid_standard: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "file_to_standard"]
pub struct InsertableFileToStandard {
    pub uuid_file: Uuid,
    pub uuid_standard: Uuid,
}

impl From<FileData> for InsertableFile {
    fn from(date_file: FileData) -> Self {
        let FileData {
            uuid_file_parent,
            hash,
            uuid_user,
            filename,
            id_ext,
            filesize,
            path_file,
            ..
        } = date_file;

        // let id_ext = 1; // get_ext_id(&path_file);
        // let hash = Vec::from("76738cf561df624bff0de7151eec68c1d40a56c76a8f6859e09c799a251468ac");
        // let filesize = 156.5; // get_file_size(&path_file);
        // let uuid_user_create= "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b".parse().unwrap();
        // let default_hash = Vec::from("0".as_bytes());
        // let hash = hex_to_bytes(hash.as_str()).unwrap_or(default_hash);

        Self {
            uuid: Uuid::new_v4(),
            uuid_file_parent,
            hash,
            uuid_user,
            filename,
            id_ext,
            filesize,
            path_file,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<File> for SlimFile {
    fn from(file: File) -> Self {
        let File {
            uuid,
            filename,
            filesize,
            path_file,
            ..
        } = file;

        Self {
            uuid,
            filename,
            filesize,
            path_file,
        }
    }
}

impl From<FileToModelData> for InsertableFileToComponent {
    fn from(data_file_to_model: FileToModelData) -> Self {
        let FileToModelData {
            uuid_file,
            uuid,
            ..
        } = data_file_to_model;

        let uuid_component = uuid;

        Self {
            uuid_file,
            uuid_component,
        }
    }
}

impl From<FileToModelData> for InsertableFileToModification {
    fn from(data_file_to_model: FileToModelData) -> Self {
        let FileToModelData {
            uuid_file,
            uuid,
            ..
        } = data_file_to_model;

        let uuid_modification = uuid;

        Self {
            uuid_file,
            uuid_modification,
        }
    }
}
