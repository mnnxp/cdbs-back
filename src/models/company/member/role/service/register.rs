use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::role::model::{
    InsertableRoleMemberTranslateList,
    IptRoleMemberData,
    RoleMemberTranslateList,
    RoleMember
};
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Create company member role 
pub(crate) fn create_role_member(
    logged_user_uuid: &Uuid,
    data: &IptRoleMemberData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::role_member_translate_list::dsl::*;

    check_is_owner_with_err(
        logged_user_uuid,
        &data.company_uuid,
        conn,
    )?;

    // get roles for target company
    let company_roles_ids = super::list::get_company_roles_ids(
        &data.company_uuid,
        conn
    )?;

    let flag_found_role_member = role_member_translate_list
        .filter(lang_id.eq(&data.lang_id)
        .and(role_member_id.eq_any(company_roles_ids))
        .and(name.eq(&data.name)))
        .select(role_member_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_role_member START SEARCH ={:?}", flag_found_role_member);

    match flag_found_role_member {
        0 => {
            let new_role_member_id = {
                use crate::schema::role_member_list::dsl::*;

                let new_role_member: RoleMember = diesel::insert_into(role_member_list)
                    .values(company_uuid.eq(&data.company_uuid))
                    .get_result(conn)?;

                new_role_member.id
            };

            let data = InsertableRoleMemberTranslateList {
                role_member_id: new_role_member_id,
                lang_id: data.lang_id,
                name: data.name.to_string(),
            };
            let inserted_data: RoleMemberTranslateList = diesel::insert_into(role_member_translate_list)
                .values(&data)
                .get_result(conn)?;

            debug!("Add new role member name, id: {:#?}", inserted_data);

            Ok(inserted_data.role_member_id)
        },
        1.. => {
            debug!("This role member name is already there. Id: {}", flag_found_role_member);

            Ok(flag_found_role_member)
        },
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
