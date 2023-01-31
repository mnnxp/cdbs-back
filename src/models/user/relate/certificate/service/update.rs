use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::certificate::model::IptUpdateUserCertificateData;
use diesel::prelude::*;
use uuid::Uuid;

/// Update user certificate desctiption
pub(crate) fn update_certificate_description(
    logged_user_uuid: &Uuid,
    data: &IptUpdateUserCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::user_certificate_ref::dsl as user_certificate_ref;

    // update column description
    let res = diesel::update(user_certificate_ref::user_certificate_ref
        .filter(user_certificate_ref::user_uuid.eq(logged_user_uuid)
        .and(user_certificate_ref::file_uuid.eq(&data.file_uuid)
        .and(user_certificate_ref::description.ne(&data.description)))))
        .set(user_certificate_ref::description.eq(data.description.to_string()))
        .execute(conn);

    match res {
        Ok(x) => {
            if x > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Err(err) => {
            debug!("Failed update data: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed update data".to_string()
            ))
        },
    }
}
