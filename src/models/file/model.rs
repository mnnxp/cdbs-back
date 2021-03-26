// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;
use crate::models::file::util::hex_to_bytes;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct File {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_file: Uuid,
    #[graphql(skip)]
    pub hash: Vec<u8>,
    pub uuid_user_create: Uuid,
    pub created_at: NaiveDateTime,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

#[derive(Debug, Insertable)]
#[table_name = "file_ref"]
pub struct InsertableFile {
    pub uuid: Uuid,
    pub uuid_file: Uuid,
    pub hash: Vec<u8>,
    pub uuid_user_create: Uuid,
    pub created_at: NaiveDateTime,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct FileData {
    pub uuid_file: Uuid,
    pub hash: String,
    pub uuid_user_create: Uuid,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimFile {
    pub uuid: Uuid,
    pub filename: String,
    pub filesize: f64,
    pub path_file: String,
}

impl From<FileData> for InsertableFile {
    fn from(date_file: FileData) -> Self {
        let FileData {
            uuid_file,
            hash,
            uuid_user_create,
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
        let default_hash = Vec::from("0".as_bytes());
        let hash = hex_to_bytes(hash.as_str()).unwrap_or(default_hash);

        Self {
            uuid: Uuid::new_v4(),
            uuid_file,
            hash,
            uuid_user_create,
            created_at: chrono::Local::now().naive_local(),
            filename,
            id_ext,
            filesize,
            path_file,
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
