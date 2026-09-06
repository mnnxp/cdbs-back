use crate::database::{get_conn, PooledConnection};
use crate::graphql::handler::extract_client_domain;
use crate::models::relate_ref::file::model::SlimFile;
use crate::models::relate_ref::program::model::Program;
use crate::models::user::model::ShowUserShort;
use async_graphql::{Context, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Full data about the file uploaded to CADBase storage.
/// And data for retrieving a file from the storage.
#[derive(Debug, Clone)]
pub(crate) struct ShowFileRelatedData {
    /// File UUID
    pub(crate) uuid: Uuid,
    /// File name
    pub(crate) filename: String,
    /// File revision number
    pub(crate) revision: i32,
    /// Commit message (comment on the file or its revision)
    pub(crate) commit_msg: String,
    /// UUID of parent file
    pub(crate) parent_file_uuid: Uuid,
    /// Data about the user who owns the file
    pub(crate) owner_user: ShowUserShort,
    /// Estimated data content type
    pub(crate) content_type: String,
    /// File size in bytes
    pub(crate) filesize: i64,
    /// Software associated with the file (to open the file)
    pub(crate) program: Program,
    /// File creation date
    pub(crate) created_at: NaiveDateTime,
    /// Date the file description was updated
    pub(crate) updated_at: NaiveDateTime,
}

#[Object]
impl ShowFileRelatedData {
    /// File UUID
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// File name
    async fn filename(&self) -> &String {
        &self.filename
    }

    /// File revision number
    async fn revision(&self) -> i32 {
        self.revision
    }

    /// Commit message (comment on the file or its revision)
    async fn commit_msg(&self) -> &String {
        &self.commit_msg
    }

    /// UUID of parent file
    async fn parent_file_uuid(&self) -> &Uuid {
        &self.parent_file_uuid
    }

    /// Data about the user who owns the file
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Estimated data content type
    async fn content_type(&self) -> &String {
        &self.content_type
    }

    /// File size in bytes
    async fn filesize(&self) -> i64 {
        self.filesize
    }

    /// Software associated with the file (to open the file)
    async fn program(&self) -> &Program {
        &self.program
    }

    /// Hash of the file calculated with BLAKE3 (there are plans to abandon this field)
    async fn hash(&self, ctx: &Context<'_>) -> String {
        let conn: &mut PooledConnection = &mut get_conn(ctx).expect("Error get conn to DB");
        SlimFile::encode_hash(&self.uuid, conn).expect("Error get encode hash (slim file)")
    }

    /// Hash of the file calculated with Sha256 (cryptographic hash function)
    async fn sha256_hash(&self, ctx: &Context<'_>) -> String {
        let conn: &mut PooledConnection = &mut get_conn(ctx).expect("Error get conn to DB");
        SlimFile::encode_sha256_hash(&self.uuid, conn).expect("Error get encode hash (slim file)")
    }

    /// Pre-signed URL to download the file
    async fn download_url(&self, ctx: &Context<'_>) -> String {
        let conn: &mut PooledConnection = &mut get_conn(ctx).expect("Error get conn to DB");
        SlimFile::get_download_string(&self.uuid, &extract_client_domain(ctx), conn)
            .expect("Error get download string (slim file)")
    }

    /// File creation date
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date the file description was updated
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}
