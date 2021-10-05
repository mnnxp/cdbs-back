use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_pool, get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::file;

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
    ) -> ServiceResult<String> {
        let conn: &PooledConnection = &get_conn(cxt)?;

        // authorization check
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        file::service::list::get_url_file_by_uuid(
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
    ) -> ServiceResult<i32> {
        let pool = get_pool(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        if file_uuids.is_empty() {
            return Ok(0) // <-- Not found uuids, just return 0
        }

        file::service::update::confirm_upload(
            &logged_user_uuid,
            &file_uuids,
            pool
        ).await
    }

    /// Delete file in storage
    async fn delete_file(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use file::service::delete::delete_file_with_check_by_uuid;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let pool = get_pool(cxt)?;

        delete_file_with_check_by_uuid(
            &logged_user_uuid,
            &file_uuid,
            &pool
        ).await
    }
}
