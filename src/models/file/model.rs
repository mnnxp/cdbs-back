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
    pub content_type: String,
    pub id_ext: i32,
    pub filesize: i32,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "file_ref"]
pub struct ShowFile {
    pub uuid: Uuid,
    pub uuid_file_parent: Uuid,
    pub uuid_user: Uuid,
    pub filename: String,
    pub content_type: String,
    pub id_ext: i32,
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
    async fn content_type(&self) -> &String {
        &self.content_type
    }
    async fn id_ext(&self) -> &i32 {
        &self.id_ext
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
    pub content_type: String,
    pub id_ext: i32,
    pub filesize: i32,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// #[derive(Debug, Deserialize, Clone, InputObject)]
// pub struct IptFileData {
//     pub uuid_file_parent: ID,
//     pub hash:  Vec<u8>,
//     pub uuid_user: ID,
//     pub filename: String,
//     pub content_type: String,
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
    pub content_type: String,
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

impl From<FileData> for InsertableFile {
    fn from(date_file: FileData) -> Self {
        let FileData {
            uuid_file_parent,
            hash,
            uuid_user,
            filename,
            content_type,
            id_ext,
            filesize,
            path_file,
            ..
        } = date_file;

        Self {
            uuid: Uuid::new_v4(),
            uuid_file_parent,
            hash,
            uuid_user,
            filename,
            content_type,
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
