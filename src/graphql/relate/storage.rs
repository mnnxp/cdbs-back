use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_pool, get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::model::TargetUser;
use crate::models::relate_ref::file;
use crate::storage::wrapper::storage_access::get_user_storage_access;

#[derive(Default)]
pub struct StorageQuery;
#[derive(Default)]
pub struct StorageMutation;

#[Object]
impl StorageQuery {
    /// Presigned URL for download file from storage
    async fn presigned_url(
        &self, cxt: &Context<'_>,
        file_uuid: String
    ) -> ServiceResult<String> {
        let conn: &PooledConnection = &get_conn(cxt)?;

        // authorization check
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let target_file_uuid = Uuid::parse_str(&file_uuid).unwrap();

        Ok(file::service::list::get_url_file_by_uuid(
            &logged_user_uuid,
            &target_file_uuid,
            conn,
        )?)
    }
}

#[Object]
impl StorageMutation {
    /// Сonfirmation of successful upload of files to storage
    async fn upload_completed(&self, cxt: &Context<'_>, file_id: String) -> ServiceResult<i32> {
        let pool = get_pool(cxt)?;

        let target_user = TargetUser::from(&crate::models::user::get_logged_user_uuid(cxt, true)?);

        Ok(file::service::update::confirm_upload(&target_user, &file_id, pool).await?)
    }

    async fn update_storage_access(
        &self, cxt: &Context<'_>,
    ) -> ServiceResult<bool> {
        let pool = get_pool(cxt)?;
        // let conn = pool.get().unwrap();

        // storage access data
        match get_user_storage_access(
            TargetUser::from(&crate::models::user::get_logged_user_uuid(cxt, true)?),
            pool
        ).await {
            Ok(data) => {
                debug!("New storage access data: {:?}", data);
                Ok(true)
            },
            Err(err) => {
                debug!("Fail get new storage access data: {:?}", err);
                Ok(false)
            }
        }
    }
}
