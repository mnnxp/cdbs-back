use crate::errors::ServiceResult;
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::relate_ref::file::{
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет аватар компании. Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла изображения.
pub(crate) fn update_favicon(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    // check access user for company
    check_is_owner_with_err(
        logged_user_uuid,
        target_company_uuid,
        conn
    )?;

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::CompanyFavicon(*target_company_uuid),
        filename,
        conn
    )?;

    // change image uuid for company
    change_image_uuid(
        target_company_uuid,
        &slim_file.uuid,
        conn,
    );

    let upload_url = upload_presigned_url(
        &StorageAccess::from_env(),
        &slim_file.path_file,
    )?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}

/// Change image file uuid for company
fn change_image_uuid (
    company_uuid: &Uuid,
    set_image_uuid: &Uuid,
    conn: &mut PgConnection,
) -> bool {
    use crate::schema::company_ref::dsl as company_ref;

    let res = diesel::update(company_ref::company_ref)
        .filter(company_ref::uuid.eq(company_uuid))
        .set(company_ref::image_file_uuid.eq(set_image_uuid))
        .execute(conn);

    debug!("Updating favicon image uuid: {:?}", res);

    matches!(res, Ok(x) if x > 0)
}
