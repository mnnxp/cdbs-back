use crate::auth::access::invalidate_access;
use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::member::model::IptCompanyMemberData;
use crate::models::company::member::role::util::check_role_of_company;
use diesel::prelude::*;
use uuid::Uuid;

/// Изменяет тип роли участника компании.
pub(crate) fn change_role_member(
    logged_user_uuid: &Uuid,
    data: &IptCompanyMemberData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    use crate::schema::company_member_list::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        &data.company_uuid,
        AccessOperation::Manage,
        conn,
    )?;
    invalidate_access(&data.user_uuid, AccessEntity::Company, &data.company_uuid);

    // return error if not found role
    check_role_of_company(&data.company_uuid, data.role_id, conn)?;

    let res_update = diesel::update(company_member_list)
        .filter(
            company_uuid
                .eq(&data.company_uuid)
                .and(user_uuid.eq(&data.user_uuid).and(role_id.ne(&data.role_id))),
        )
        .set((
            role_id.eq(data.role_id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn);

    match res_update {
        Ok(1_usize) => {
            debug!("Update role member: {:?}", res_update);
            Ok(true)
        }
        Ok(x) => {
            debug!("Role member duplicate data: {:?}", x);
            Ok(false)
        }
        Err(err) => {
            debug!("Failed update role member: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedUpdateRoleMember))
        }
    }
}
