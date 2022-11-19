use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::certificate::model::DelUserCertificateData;
use crate::models::relate_ref::file::service::delete::{
    delete_file_by_uuid, delete_file_by_uuids
};
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete user certificate desctiption
pub(crate) fn del_certificate_description(
    logged_user_uuid: &Uuid,
    data: &DelUserCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // delete row user certificate
    let file_uuid = diesel::delete(user_certificate_ref::user_certificate_ref
        .filter(user_certificate_ref::user_uuid.eq(logged_user_uuid)
        .and(user_certificate_ref::file_uuid.eq(&data.file_uuid))))
        .returning(user_certificate_ref::file_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed remove certificate data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // delete file rows and file in storage
    delete_file_by_uuid(&file_uuid, conn)
}

/// Set the delete flags for all user certificates
pub(crate) fn delete_user_certificates(
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let del_file_uuids = diesel::delete(user_certificate_ref::user_certificate_ref
        .filter(user_certificate_ref::user_uuid.eq(user_uuid)))
        .returning(user_certificate_ref::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets certificates of user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
