use crate::errors::ServiceResult;
use crate::models::user::model::UserShort;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn update_favicon(
    target_user_uuid: &Uuid,
    filename: &str,
    conn: &PgConnection,
) -> ServiceResult<UploadFile> {
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

    // change image uuid for user
    change_image_uuid(
        &user_short.uuid,
        &slim_file.uuid,
        conn,
    );

    let upload_url = upload_presigned_url(
        &StorageAccess::get(conn)?,
        &slim_file.path_file,
    )?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}

/// Change image file uuid for user
fn change_image_uuid (
    user_uuid: &Uuid,
    set_image_uuid: &Uuid,
    conn: &PgConnection,
) -> bool {
    use crate::schema::user_ref::dsl as user_ref;

    let res = diesel::update(user_ref::user_ref)
        .filter(user_ref::uuid.eq(user_uuid))
        .set(user_ref::image_file_uuid.eq(set_image_uuid))
        .execute(conn);

    debug!("Updating favicon image uuid: {:?}", res);

    matches!(res, Ok(x) if x > 0)
}
