use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::program::model::Program;

// list for insert data in related tables
#[derive(Deserialize, Clone, Debug)]
pub enum ListObject {
    User(Uuid),
    UserCertificate(Uuid), // <-- addiction_uuid auth user
    Company(Uuid),
    CompanyFavicon(Uuid),
    CompanyCertificate(Uuid),
    Component(Uuid),
    ComponentModification(Uuid),
    ComponentModificationSet(Uuid),
    Standard(Uuid),
}

impl ListObject {
    /// Clone Uuid from enum
    pub(crate) fn get_uuid(&self) -> Uuid {
        match self {
            ListObject::User(uuid_object) => *uuid_object,
            ListObject::UserCertificate(uuid_object) => *uuid_object,
            ListObject::Company(uuid_object) => *uuid_object,
            ListObject::CompanyFavicon(uuid_object) => *uuid_object,
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

#[derive(Identifiable, Queryable, Clone, Debug)]
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
    pub path_file: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub is_delete: bool,
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
            // maybe uuid from component, modification, standard, user etc
            Uuid::to_simple(object.get_uuid()),
            // user_uuid
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
            is_delete: false,
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
    /// linked object, to create a new name in the storage (file_path)
    pub object: ListObject,
    /// sanitizer filename with sanitize_filename::sanitize(&filename)
    pub filename: String,
    /// get id for extension with find_id_ext(filename, conn)
    pub id_ext: i32,
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

#[derive(Serialize, Debug, SimpleObject)]
pub struct UploadFile {
    pub file_uuid: Uuid,
    pub filename: String,
    pub upload_url: String,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct ShowFileRelatedData {
    pub uuid: Uuid,
    pub filename: String,
    pub parent_file_uuid: Uuid,
    pub owner_user: ShowUserShort,
    pub content_type: String,
    pub filesize: i64,
    pub program: Program,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, SimpleObject, Clone, Default, Debug)]
pub struct DownloadFile {
    pub uuid: Uuid,
    pub filename: String,
    pub filesize: i64,
    pub download_url: String,
}

#[derive(Debug)]
pub(crate) struct FileByExtArg {
    pub(crate) ext_id: i32,
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

impl FileByExtArg {
    /// Get struct for get 1th image
    pub(crate) fn image() -> Self {
        Self{
            ext_id: 2, // (image)
            limit: 1,
            offset: 0,
        }
    }
}
