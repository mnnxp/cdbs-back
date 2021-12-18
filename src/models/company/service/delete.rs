use crate::errors::{ServiceResult, ServiceError};
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

// todo!(в будущем, сделать корзину)
// 1. проверить пользователя на владение компанией
// 2. установить новый флаг
// 3. через 30 дней удалить все данные компании

/// Delete all data company
pub(crate) fn del_company(
    logged_user_uuid: &Uuid,
    del_company_uuid: &Uuid,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    diesel::delete(company_ref::company_ref
        .filter(company_ref::user_uuid.eq(logged_user_uuid) // <-- only companies the user
        .and(company_ref::uuid.eq(del_company_uuid))))
        .returning(company_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete company: {:?}", err);
            ServiceError::InternalServerError
        })
}
