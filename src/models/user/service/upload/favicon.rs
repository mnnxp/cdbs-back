use crate::errors::ServiceResult;
use crate::models::user::model::UserShort;
use crate::models::relate_ref::file::model::{
    ListObject, IptPreliminaryFileData, PreliminaryFileData
};
use crate::models::relate_ref::file as file;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn update_favicon(
    target_user_uuid: &Uuid,
    file_data: &IptPreliminaryFileData,
    conn: &PgConnection,
) -> ServiceResult<String> {

    // let content_sha1: String = file_data.sha1.clone();

    let user_short = UserShort::get_by_uuid(
        target_user_uuid,
        conn
    )?;

    let preliminary_file_data = PreliminaryFileData::from_ipt_preliminary_file_data(
        target_user_uuid.to_owned(),
        user_short.image_file_uuid,
        ListObject::User(user_short.uuid),
        file_data.to_owned(),
        conn
    )?;

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    upload_presigned_url(
        &UserStorageAccess::get(
            target_user_uuid,
            conn
        )?,
        &slim_file.path_file,
        // &content_sha1,
    )
}
