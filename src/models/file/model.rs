use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct File {
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

#[Object]
impl ShowFile {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_file_parent(&self) -> ID {
        self.uuid_file_parent.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn filename(&self) -> &String {
        &self.filename
    }
    async fn id_ext(&self) -> &i32 {
        &self.id_ext
    }
    async fn value_ext(&self) -> &String {
        &self.value_ext
    }
    async fn filesize(&self) -> &i32 {
        &self.filesize
    }
    async fn path_file(&self) -> &String {
        &self.path_file
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
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

// #[derive(Debug, Deserialize, Clone, InputObject)]
// pub struct FileData {
//     pub uuid_file_parent: ID,
//     pub hash:  Vec<u8>,
//     pub uuid_user: ID,
//     pub filename: String,
//     pub id_ext: i32,
//     pub filesize: i32,
//     pub path_file: String,
// }

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
    pub uuid_file: Uuid,
    pub uuid: Uuid,
}

// #[Object]
// impl FileToModel {
//     async fn uuid_file(&self) -> ID {
//         self.uuid_file.into()
//     }
//     async fn uuid(&self) -> ID {
//         self.uuid.into()
//     }
// }

// Structures for Component
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct FileToComponent {
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

impl From<FileToModel> for InsertableFileToComponent {
    fn from(data_file_to_model: FileToModel) -> Self {
        let FileToModel {
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

impl From<FileToModel> for InsertableFileToModification {
    fn from(data_file_to_model: FileToModel) -> Self {
        let FileToModel {
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
