use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// /// First uuid: object_uuid, second uuid: addiction_uuid
// #[derive(Deserialize, Clone, Debug)]
// pub struct DoubleAddiction(Uuid,Uuid);
//
// impl DoubleAddiction {
//     /// Clone object uuid of DoubleAddiction
//     pub fn get_object_uuid(&self) -> Uuid {
//         self.0
//     }
//
//     /// Clone addiction uuid of DoubleAddiction
//     pub fn get_addiction_uuid(&self) -> Uuid {
//         self.1
//     }
// }

// list for insert data in related tables
#[derive(Deserialize, Clone, Debug)]
pub enum ListObject {
    User(Uuid),
    UserCertificate(Uuid), // <-- addiction_uuid auth user
    Company(Uuid),
    CompanyCertificate(Uuid),
    Component(Uuid),
    ComponentModification(Uuid),
    ComponentModificationSet(Uuid),
    Standard(Uuid),
}

impl ListObject {
    /// Clone Uuid from enum
    pub fn get_uuid(&self) -> Uuid {
        match self {
            ListObject::User(uuid_object) => *uuid_object,
            ListObject::UserCertificate(uuid_object) => *uuid_object,
            ListObject::Company(uuid_object) => *uuid_object,
            ListObject::CompanyCertificate(uuid_object) => *uuid_object,
            ListObject::Component(uuid_object) => *uuid_object,
            ListObject::ComponentModification(uuid_object) => *uuid_object,
            ListObject::ComponentModificationSet(uuid_object) => *uuid_object,
            ListObject::Standard(uuid_object) => *uuid_object,
        }
    }
}

#[derive(Debug, Queryable)]
pub struct File {
    pub uuid: Uuid,
    pub parent_file_uuid: Uuid,
    pub hash: Vec<u8>,
    pub user_uuid: Uuid,
    pub filename: String,
    pub content_type: String,
    pub id_ext: i32,
    pub filesize: i64,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "file_ref"]
pub struct ShowFile {
    pub uuid: Uuid,
    pub parent_file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub filename: String,
    pub content_type: String,
    pub id_ext: i32,
    pub filesize: i64,
    // pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ShowFile {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn parent_file_uuid(&self) -> ID {
        self.parent_file_uuid.into()
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
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
    async fn filesize(&self) -> &i64 {
        &self.filesize
    }
    // async fn path_file(&self) -> &String {
    //     &self.path_file
    // }
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
    pub parent_file_uuid: Uuid,
    pub hash: Vec<u8>,
    pub user_uuid: Uuid,
    pub filename: String,
    pub content_type: String,
    pub id_ext: i32,
    pub filesize: i64,
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


impl From<PreliminaryFileData> for InsertableFile {
    fn from(data: PreliminaryFileData) -> Self {
        let PreliminaryFileData {
            parent_file_uuid,
            object,
            user_uuid,
            filename,
            id_ext,
            content_type,
            // filesize,
            ..
        } = data;

        let new_file_uuid = Uuid::new_v4();

        // creating a filename for the storage
        let path_file = format!("{}/{}",
            Uuid::to_simple(object.get_uuid()), // <- maybe uuid from component, modification, standard, user etc.
            // user_uuid,
            Uuid::to_simple(new_file_uuid),
        );

        Self {
            uuid: new_file_uuid,
            parent_file_uuid,
            hash: Vec::new(),
            user_uuid,
            filename,
            content_type,
            id_ext,
            filesize: 0_i64,
            path_file,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// For generate file pre-entry in the database
#[derive(Deserialize, Debug)]
pub struct PreliminaryFileData {
    pub parent_file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub object: ListObject, // <-- linked object, to create a new name in the storage (file_path)
    pub filename: String, // <-- sanitizer filename with sanitize_filename::sanitize(&filename)
    pub id_ext: i32, // <-- get id for extension with find_id_ext(filename, conn)
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
pub struct FileData {
    pub parent_file_uuid: Option<Uuid>,
    pub hash: Option<Vec<u8>>,
    pub user_uuid: Option<Uuid>,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub id_ext: Option<i32>,
    pub filesize: Option<i64>,
    pub path_file: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "file_ref"]
pub struct SlimFile {
    pub uuid: Uuid,
    pub filename: String,
    pub filesize: i64,
    pub path_file: String,
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

#[derive(Serialize, Debug)]
pub struct UploadFile {
    pub file_uuid: Uuid,
    pub filename: String,
    pub upload_url: String,
}

#[Object]
impl UploadFile {
    async fn file_uuid(&self) -> ID {
        self.file_uuid.into()
    }
    async fn filename(&self) -> &String {
        &self.filename
    }
    async fn upload_url(&self) -> &String {
        &self.upload_url
    }
}

#[derive(Serialize, Debug)]
pub struct DownloadFile {
    pub uuid: Uuid,
    pub filename: String,
    pub filesize: i64,
    pub download_url: String,
}

#[Object]
impl DownloadFile {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn filename(&self) -> &String {
        &self.filename
    }
    async fn filesize(&self) -> &i64 {
        &self.filesize
    }
    async fn download_url(&self) -> &String {
        &self.download_url
    }
}
