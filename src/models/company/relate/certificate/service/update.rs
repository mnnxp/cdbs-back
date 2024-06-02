use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::certificate::model::IptUpdateCompanyCertificateData;
use crate::models::company::access::util::check_company_access;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет описание сертификата компании.
/// Возвращает true, если изменение прошло успешно, и false, если описание сертификата уже установлено.
pub(crate) fn update_certificate_description(
    logged_user_uuid: &Uuid,
    data: &IptUpdateCompanyCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_certificate_ref::dsl as company_certificate_ref;

    let need_access_level = 1; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    // update column description
    let res = diesel::update(company_certificate_ref::company_certificate_ref
        .filter(company_certificate_ref::company_uuid.eq(&data.company_uuid)
        .and(company_certificate_ref::file_uuid.eq(&data.file_uuid)
        .and(company_certificate_ref::description.ne(&data.description)))))
        .set(company_certificate_ref::description.eq(data.description.to_string()))
        .execute(conn);

    match res {
        Ok(x) => Ok(x > 0),
        Err(err) => {
            debug!("Failed update data: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed update data".to_string()
            ))
        },
    }
}
