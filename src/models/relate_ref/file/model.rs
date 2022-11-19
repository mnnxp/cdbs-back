use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::program::model::Program;

// list for insert data in related tables
#[derive(Deserialize, Clone, Debug)]
pub(crate) enum ListObject {
    /// For bind file to the user
    User(Uuid),
    /// For bind file certificate to the user
    UserCertificate(Uuid),
    /// For bind file to the company
    Company(Uuid),
    /// For change file favicon company
    CompanyFavicon(Uuid),
    /// For bind file certificate to the company
    CompanyCertificate(Uuid),
    /// For bind file to the component
    Component(Uuid),
    /// For change file for main image (favicon) component
    ComponentFavicon(Uuid),
    /// For bind file to the component modification
    ComponentModification(Uuid),
    /// For bind file to the component modification filesset
    ComponentModificationSet(Uuid),
    /// For bind file to the standard
    Standard(Uuid),
    /// For change file for main image (favicon) standard
    StandardFavicon(Uuid),
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
            ListObject::ComponentFavicon(uuid_object) => *uuid_object,
            ListObject::ComponentModification(uuid_object) => *uuid_object,
            ListObject::ComponentModificationSet(uuid_object) => *uuid_object,
            ListObject::Standard(uuid_object) => *uuid_object,
            ListObject::StandardFavicon(uuid_object) => *uuid_object,
        }
    }
}

#[derive(Debug, Queryable)]
pub(crate) struct File {
    pub(crate) uuid: Uuid,
    // pub(crate) parent_file_uuid: Uuid,
    // pub(crate) hash: Vec<u8>,
    // pub(crate) user_uuid: Uuid,
    pub(crate) filename: String,
    // pub(crate) content_type: String,
    // pub(crate) id_ext: i32,
    pub(crate) filesize: i64,
    pub(crate) path_file: String,
    // pub(crate) created_at: NaiveDateTime,
    // pub(crate) updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = file_ref)]
pub(crate) struct ShowFile {
    pub(crate) uuid: Uuid,
    pub(crate) parent_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) content_type: String,
    pub(crate) id_ext: i32,
    pub(crate) filesize: i64,
    // pub(crate) path_file: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = file_ref)]
pub(crate) struct InsertableFile {
    pub(crate) uuid: Uuid,
    pub(crate) parent_file_uuid: Uuid,
    pub(crate) hash: Vec<u8>,
    pub(crate) user_uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) content_type: String,
    pub(crate) id_ext: i32,
    pub(crate) filesize: i64,
    pub(crate) path_file: String,
    pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
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
            Uuid::simple(object.get_uuid()),
            // user_uuid
            Uuid::simple(new_file_uuid),
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
pub(crate) struct PreliminaryFileData {
    pub(crate) parent_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    /// linked object, to create a new name in the storage (file_path)
    pub(crate) object: ListObject,
    /// sanitizer filename with sanitize_filename::sanitize(&filename)
    pub(crate) filename: String,
    /// get id for extension with find_id_ext(filename, conn)
    pub(crate) id_ext: i32,
    pub(crate) content_type: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FileData {
    pub(crate) parent_file_uuid: Option<Uuid>,
    pub(crate) hash: Option<Vec<u8>>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) filename: Option<String>,
    pub(crate) content_type: Option<String>,
    pub(crate) id_ext: Option<i32>,
    pub(crate) filesize: Option<i64>,
    pub(crate) path_file: Option<String>,
}

#[derive(Identifiable, Queryable, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = file_ref)]
pub(crate) struct SlimFile {
    pub(crate) uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) filesize: i64,
    pub(crate) path_file: String,
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
pub(crate) struct UploadFile {
    pub(crate) file_uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) upload_url: String,
}

#[derive(Debug, SimpleObject, Clone)]
pub(crate) struct ShowFileRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) parent_file_uuid: Uuid,
    pub(crate) owner_user: ShowUserShort,
    pub(crate) content_type: String,
    pub(crate) filesize: i64,
    pub(crate) program: Program,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Serialize, SimpleObject, Clone, Default, Debug)]
pub(crate) struct DownloadFile {
    pub(crate) uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) filesize: i64,
    pub(crate) download_url: String,
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
