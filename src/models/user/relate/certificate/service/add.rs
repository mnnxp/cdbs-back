use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::{
    commit::Commit,
    model::{ListObject, UploadFile},
    service::register::preregister_file,
};
use crate::models::user::certificate::model::{
    InsertableUserCertificate, IptUserCertificateData, UserCertificate,
};
use crate::schema::user_certificate_ref::dsl::*;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

/// Загрузка нового сертификата пользователя.
/// Возвращает структуру с предварительно подписанным URL-адресом для загрузки файла (сертификата).
pub(crate) fn add_certificate(
    logged_user_uuid: &Uuid,
    cert_data: &IptUserCertificateData,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<UploadFile> {
    // update data validation
    if cert_data.description.chars().count() > 500 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(500)));
    }

    let slim_file = preregister_file(
        logged_user_uuid,
        ListObject::UserCertificate(*logged_user_uuid),
        &cert_data.filename,
        &Commit::create_commit("Upload certificate for user", conn)?,
        conn,
    )?;

    let new_user_certificate = InsertableUserCertificate {
        file_uuid: slim_file.uuid,
        user_uuid: *logged_user_uuid,
        description: cert_data.description.clone(),
    };

    let user_inserted_certificate = diesel::insert_into(user_certificate_ref)
        .values(new_user_certificate)
        .get_result::<UserCertificate>(conn)
        .map_err(|err| {
            debug!("Failed insert certificate data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    debug!("User inserted certificate: {:?}", user_inserted_certificate);

    let upload_url = upload_presigned_url(&StorageAccess::from_env(), &slim_file.path_file, domain)?;

    Ok(UploadFile {
        file_uuid: slim_file.uuid,
        filename: slim_file.filename,
        upload_url,
    })
}
