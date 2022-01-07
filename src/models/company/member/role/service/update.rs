use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::role::model::IptUpdateNameRoleData;
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Change name role company
pub(crate) fn change_name_role_company(
    logged_user_uuid: &Uuid,
    data: &IptUpdateNameRoleData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::role_member_translate_list::dsl as role_member_tl;

    check_is_owner_with_err(
        logged_user_uuid,
        &data.company_uuid,
        conn,
    )?;

    // update column name
    let res = diesel::update(role_member_tl::role_member_translate_list
        .filter(role_member_tl::role_member_id.eq(&data.role_id)
        .and(role_member_tl::lang_id.eq(&data.lang_id)
        .and(role_member_tl::name.ne(&data.name)))))
        .set(role_member_tl::name.eq(data.name.to_string()))
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
