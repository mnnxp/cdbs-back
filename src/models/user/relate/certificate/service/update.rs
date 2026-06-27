use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::user::certificate::model::IptUpdateUserCertificateData;
use crate::schema::user_certificate_ref::dsl as user_certificate_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет описание сертификата пользователя.
/// Возвращает true, если изменение прошло успешно, и false, если описание сертификата уже установлено.
pub(crate) fn update_certificate_description(
    logged_user_uuid: &Uuid,
    data: &IptUpdateUserCertificateData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // update data validation
    if data.description.chars().count() > 500 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(500)));
    }

    // update column description
    let res = diesel::update(
        user_certificate_ref::user_certificate_ref.filter(
            user_certificate_ref::user_uuid.eq(logged_user_uuid).and(
                user_certificate_ref::file_uuid
                    .eq(&data.file_uuid)
                    .and(user_certificate_ref::description.ne(&data.description)),
            ),
        ),
    )
    .set(user_certificate_ref::description.eq(data.description.clone()))
    .execute(conn);

    match res {
        Ok(x) => Ok(x > 0),
        Err(err) => {
            debug!("Failed update data: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedUpdateData))
        }
    }
}
