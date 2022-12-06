use crate::errors::ServiceResult;
use crate::database::{get_pool, get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::relate_ref::file::{
    model::DownloadFile,
    service::list::get_url_by_file_uuid,
    service::update::confirm_upload,
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
    /// Presigned URL for download file from storage
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
}

#[Object]
impl StorageMutation {
    /// Сonfirmation of successful upload of files to storage
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

    /// Delete file in storage
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
