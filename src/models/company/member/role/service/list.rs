use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::access::util::check_company_access;
use crate::models::company::member::role::model::RoleMemberAndRelatedData;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

/// Gets IDs roles for company by uuid
/// Warning: fn without check access and not use for API GraphQL
pub(super) fn get_company_roles_ids(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<i32>> {
    use crate::schema::role_member_list::dsl::*;

    let res = role_member_list
        .filter(company_uuid.eq(target_company_uuid))
        .select(id)
        .load::<i32>(conn);

    match res {
        Ok(x) => Ok(x),
        Err(err) => {
            debug!("Error get roles for target company: {:?}", err);
            Err(get_err_msg(ErrorMessage::NotFoundAccessForRole))
        }
    }
}

/// Возвращает агрегированные данные ролей для участников компании.
pub(crate) fn get_roles_for_company(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RoleMemberAndRelatedData>> {
    use crate::schema::role_member_list::dsl::*;

    let need_access_level = 3; // todo!(create enum for manage access level)

    if !check_company_access(
        logged_user_uuid,
        target_company_uuid,
        &need_access_level,
        conn,
    )? {
        // return error if user not have access level
        return Err(get_err_msg(ErrorMessage::AccessDenied));
    }

    let roles_ids = role_member_list
        .filter(company_uuid.eq(target_company_uuid))
        .select(id)
        .load::<i32>(conn);

    match roles_ids {
        Ok(ref rids) => RoleMemberAndRelatedData::get_roles_by_ids(rids, set_lang_id, conn),
        Err(err) => {
            debug!("Not found roles for company: {:?}", err);
            Err(get_err_msg(ErrorMessage::NotFoundRolesForCompany))
        }
    }
}
