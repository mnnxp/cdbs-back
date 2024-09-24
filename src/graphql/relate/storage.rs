use crate::errors::ServiceResult;
use crate::database::{get_pool, get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::relate_ref::file::{
    model::{DownloadFile, ShowFileRelatedData},
    service::list::{get_url_by_file_uuid, get_revisions_by_file_uuid},
    service::update::{confirm_upload, set_active_revision_by_uuid},
    service::delete::delete_file_with_check_by_uuid,
};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StorageQuery;
#[derive(Default)]
pub struct StorageMutation;

#[Object]
impl StorageQuery {
    /// Returns a pre-signed URL for downloading a file from storage.
    /// Works only if the file supports versioning, is associated with:
    /// a component, a modification of a component, a set of files, or a standard.
    async fn presigned_url(
        &self, cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<DownloadFile> {
        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_url_by_file_uuid(
            &logged_user_uuid,
            &file_uuid,
            conn,
        )
    }

    /// Returns information about all revisions (versions) of a file.
    async fn show_file_revisions(
        &self, cxt: &Context<'_>,
        file_uuid: Uuid,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_revisions_by_file_uuid(
            &file_uuid,
            &get_logged_user_uuid(cxt, true)?,
            limit.unwrap_or(100),
            offset.unwrap_or(0),
            conn,
        )
    }
}

#[Object]
impl StorageMutation {
    /// Sets a file as successfully uploaded to the storage.
    /// After successful uploaded is confirmed, the file will be processed.
    async fn upload_completed(
        &self,
        cxt: &Context<'_>,
        file_uuids: Vec<Uuid>,
    ) -> ServiceResult<usize> {

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        if file_uuids.is_empty() {
            return Ok(0) // <-- Not found uuids, just return 0
        }

        let pool = get_pool(cxt)?;

        confirm_upload(
            &logged_user_uuid,
            &file_uuids,
            &pool
        ).await
    }

    /// Sets a specified file revision (versions) as active.
    async fn change_active_file_revision(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<bool> {
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_active_revision_by_uuid(
            &logged_user_uuid,
            &file_uuid,
            conn
        )
    }

    /// Removes the specified revision (version) of a file.
    /// If the active revision of a file is deleted, other revisions of the file will not show.
    /// After deleting the active revision without activating the other one,
    /// uploading a new file with the same name will be the solution to view other (inactive) revisions of the file.
    async fn delete_file(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<bool> {
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_file_with_check_by_uuid(
            &logged_user_uuid,
            &file_uuid,
            conn
        )
    }
}
