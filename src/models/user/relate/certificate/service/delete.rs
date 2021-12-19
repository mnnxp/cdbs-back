use crate::errors::{ServiceResult, ServiceError};
use crate::database::PgPool;
use crate::models::user::certificate::model::DelUserCertificateData;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete user certificate desctiption
pub(crate) async fn del_certificate_description(
    logged_user_uuid: &Uuid,
    data: &DelUserCertificateData,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    // delete row user certificate
    let file_uuid = diesel::delete(user_certificate_ref::user_certificate_ref
        .filter(user_certificate_ref::user_uuid.eq(logged_user_uuid)
        .and(user_certificate_ref::file_uuid.eq(&data.file_uuid))))
        .returning(user_certificate_ref::file_uuid)
        .get_result::<Uuid>(&conn)
        .map_err(|err| {
            debug!("Failed remove certificate data: {:?}", err);
            ServiceError::BadRequest("Failed remove certificate data".to_string())
        })?;

    // delete file rows and file in storage
    delete_file_by_uuid(&file_uuid, pool).await
}
