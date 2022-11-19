use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, UploadFile
};
use crate::models::relate_ref::file as file;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

/// Update company favicon
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

    let parent_image_file_uuid = get_uuid_company_image(
        target_company_uuid,
        conn
    )?;

    // Get data for write information about the file before upload to storage
    let preliminary_file_data = PreliminaryFileData::from_ipt_file_data(
        *logged_user_uuid,
        parent_image_file_uuid,
        ListObject::CompanyFavicon(*target_company_uuid),
        filename,
        conn
    );

    let slim_file = file::service::register::preregister_file(
        preliminary_file_data,
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

/// Get image file uuid for company
fn get_uuid_company_image (
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::company_ref::dsl as company_ref;

    company_ref::company_ref
        .filter(company_ref::uuid.eq(company_uuid))
        .select(company_ref::image_file_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get favicon image uuid by company: {:?}", err);
            ServiceError::InternalServerError
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
