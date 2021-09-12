use crate::errors::ServiceResult;
use crate::database::PgConn;
use crate::models::user::model::{UserShort, TargetUser};
use crate::models::relate_ref::file::model::{
    ListObject, IptPreliminaryFileData, PreliminaryFileData
};
use crate::models::relate_ref::file as file;
use crate::storage::backblaze::b2_types::UploadUrl;
// use uuid::Uuid;

pub(crate) async fn update_favicon(
    target_user: TargetUser,
    file_data: IptPreliminaryFileData,
    pool: PgConn,
) -> ServiceResult<UploadUrl> {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let content_sha1: String = file_data.sha1.clone();

    let user_short = UserShort::get_by_uuid(
        &target_user.0,
        &conn
    )?;

    let preliminary_file_data = PreliminaryFileData::from_ipt_preliminary_file_data(
        target_user.0,
        user_short.image_file_uuid,
        ListObject::User(user_short.uuid),
        file_data,
        &conn
    )?;

    let slim_file = file::service::register::register(
        preliminary_file_data,
        &conn
    )?;

    let upload_url_data = crate::storage::wrapper::upload::get_url_upload_file(
        target_user,
        pool
    ).await;

    match upload_url_data {
        Ok(upload_url_data) => Ok(UploadUrl{
            authorization: upload_url_data.authorization_token,
            file_name: slim_file.path_file,
            content_type: "b2/x-auto".to_string(),
            content_sha1,
            server_side_encryption: "AES256".to_string(),
            upload_url: upload_url_data.upload_url,
        }),
        Err(e) => Err(e),
    }
}
