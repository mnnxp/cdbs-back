use crate::errors::ServiceResult;
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет аватар пользователя. Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла изображения.
pub(crate) fn update_favicon(
    target_user_uuid: &Uuid,
    filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    let slim_file = preregister_file(
        target_user_uuid,
        ListObject::User(*target_user_uuid),
        filename,
        &Commit::create_commit("Upload favicon", conn)?,
        conn,
    )?;

    // change image uuid for user
    change_image_uuid(target_user_uuid, &slim_file.uuid, conn);

    let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file)?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}

/// Change image file uuid for user
fn change_image_uuid(user_uuid: &Uuid, set_image_uuid: &Uuid, conn: &mut PgConnection) -> bool {
    use crate::schema::user_ref::dsl as user_ref;

    let res = diesel::update(user_ref::user_ref)
        .filter(user_ref::uuid.eq(user_uuid))
        .set(user_ref::image_file_uuid.eq(set_image_uuid))
        .execute(conn);

    debug!("Updating favicon image uuid: {:?}", res);

    matches!(res, Ok(x) if x > 0)
}
