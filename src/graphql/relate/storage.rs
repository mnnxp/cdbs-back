use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::get_pool;
use crate::errors::ServiceResult;
use crate::models::user::model::TargetUser;
// use crate::models::user as user;
use crate::models::relate_ref::file;
use crate::models::relate_ref::file::model::IptPreliminaryFileData;
use crate::storage::backblaze::b2_types::UploadUrl;

#[derive(Default)]
pub struct StorageQuery;
#[derive(Default)]
pub struct StorageMutation;

#[Object]
impl StorageQuery {
    async fn presigned_url(&self, cxt: &Context<'_>, uuid_file: String) -> ServiceResult<String> {
        let pool = get_pool(cxt)?;

        // authorization check
        let target_user = TargetUser::from(&crate::models::user::get_logged_uuid_user(cxt, true)?);

        let target_uuid_file = Uuid::parse_str(&uuid_file).unwrap();

        Ok(file::service::list::get_url_file_by_uuid(target_user, target_uuid_file, pool).await?)
    }
}

#[Object]
impl StorageMutation {
    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        file_data: IptPreliminaryFileData,
    ) -> ServiceResult<UploadUrl> {
        use crate::models::user::service::upload::favicon::update_favicon;
        let pool = get_pool(cxt)?;

        let target_user = TargetUser::from(&crate::models::user::get_logged_uuid_user(cxt, true)?);

        Ok(update_favicon(target_user, file_data, pool).await?)
    }

    async fn upload_completed(&self, cxt: &Context<'_>, file_id: String) -> ServiceResult<i32> {
        let pool = get_pool(cxt)?;
        // let conn = pool.get().unwrap();

        let target_user = TargetUser::from(&crate::models::user::get_logged_uuid_user(cxt, true)?);

        Ok(file::service::update::confirm_upload(&target_user, &file_id, pool).await?)
    }
}
