use crate::auth::access::invalidate_access;
use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::model::{InsertableCompanyMember, IptCompanyMemberData};
use crate::models::company::member::role::util::check_role_of_company;
use crate::schema::company_member_list::dsl as company_member_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет участника компании (сообщества).
/// Член компании будет иметь авторизованный доступ ко всем компонентам и стандартам компании.
/// Возвращает ошибку, если этот пользователь уже является членом компании.
pub(crate) fn add_company_member(
    logged_user_uuid: &Uuid,
    data: &IptCompanyMemberData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &data.company_uuid,
        AccessOperation::Manage,
        conn,
    )?;
    invalidate_access(&data.user_uuid, AccessEntity::Company, &data.company_uuid);

    check_role_of_company(&data.company_uuid, data.role_id, conn)?;

    let check_has_member = company_member_list::company_member_list
        .filter(company_member_list::user_uuid.eq(&data.user_uuid))
        .limit(1)
        .execute(conn)
        .map_err(|err| {
            debug!("Failed delete company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match check_has_member {
        0 => {
            let insert_data: InsertableCompanyMember = data.into();
            diesel::insert_into(company_member_list::company_member_list)
                .values(&insert_data)
                .returning(company_member_list::is_enabled)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed delete company: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
        _ => Err(get_err_msg(ErrorMessage::UserHasAlreadyMemberInTheCompany)),
    }
}
