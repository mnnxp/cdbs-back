use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::certificate::model::IptUpdateCompanyCertificateData;
use crate::schema::company_certificate_ref::dsl as company_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет описание сертификата компании.
/// Возвращает true, если изменение прошло успешно, и false, если описание сертификата уже установлено.
pub(crate) fn update_certificate_description(
    logged_user_uuid: &Uuid,
    data: &IptUpdateCompanyCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // update data validation
    if data.description.chars().count() > 500 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(500)));
    }

    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &data.company_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // update column description
    let res = diesel::update(
        company_certificate_ref::company_certificate_ref.filter(
            company_certificate_ref::company_uuid
                .eq(&data.company_uuid)
                .and(
                    company_certificate_ref::file_uuid
                        .eq(&data.file_uuid)
                        .and(company_certificate_ref::description.ne(&data.description)),
                ),
        ),
    )
    .set(company_certificate_ref::description.eq(data.description.clone()))
    .execute(conn);

    match res {
        Ok(x) => Ok(x > 0),
        Err(err) => {
            debug!("Failed update data: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedUpdateData))
        }
    }
}
