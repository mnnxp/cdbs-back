use crate::errors::ServiceResult;
use crate::models::user::model::UserShort;
use crate::models::relate_ref::file::model::{ListObject, PreliminaryFileData};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn update_favicon(
    target_user_uuid: &Uuid,
    filename: &str,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let user_short = UserShort::get_by_uuid(
        target_user_uuid,
        conn
    )?;

    // Get data for write information about the file before upload to storage
    let preliminary_file_data = PreliminaryFileData::from_ipt_file_data(
        *target_user_uuid,
        user_short.image_file_uuid,
        ListObject::User(user_short.uuid),
        filename,
        conn
    );

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    upload_presigned_url(
        &StorageAccess::get(conn)?,
        &slim_file.path_file,
    )
}
