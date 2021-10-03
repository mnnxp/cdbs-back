use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::model::SlimCompany;
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
) -> ServiceResult<SlimCompany> {
    use crate::schema::company_ref::dsl::*;

    let delete_company = diesel::delete(company_ref
        .filter(user_uuid.eq(logged_user_uuid) // <-- only companies the user
        .and(uuid.eq(del_company_uuid))))
        .returning((
            uuid,
            shortname,
            is_supplier,
        ))
        .get_result::<SlimCompany>(conn);

    match delete_company {
        Ok(res) => Ok(res),
        Err(err) => {
            debug!("Failed delete company: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed delete company".to_string()
            ))
        }
    }
}
