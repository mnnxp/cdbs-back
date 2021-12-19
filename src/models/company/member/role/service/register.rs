use super::list::get_company_roles_ids;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::role::model::{
    InsertableRoleMemberTranslateList, IptRoleMemberData
};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::schema::role_member_translate_list::dsl as role_member_translate_list;
use crate::schema::role_member_list::dsl as role_member_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Create company member role
pub(crate) fn create_role_member(
    logged_user_uuid: &Uuid,
    data: &IptRoleMemberData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    check_is_owner_with_err(
        logged_user_uuid,
        &data.company_uuid,
        conn,
    )?;

    // get roles for target company
    let company_roles_ids = get_company_roles_ids(
        &data.company_uuid,
        conn
    )?;

    let flag_found = role_member_translate_list::role_member_translate_list
        .filter(role_member_translate_list::lang_id.eq(&data.lang_id)
        .and(role_member_translate_list::role_member_id.eq_any(company_roles_ids))
        .and(role_member_translate_list::name.eq(&data.name)))
        .select(role_member_translate_list::role_member_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get role_member_translate_list: {:?} ", err);
            ServiceError::InternalServerError
        })?;

    // debug!("fn create_role_member START SEARCH ={:?}", flag_found);

    match flag_found.first() {
        Some(x) => Ok(*x),
        None => {
            let new_role_member_id = diesel::insert_into(role_member_list::role_member_list)
                .values(role_member_list::company_uuid.eq(&data.company_uuid))
                .returning(role_member_list::id)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert role_member_list: {:?} ", err);
                    ServiceError::InternalServerError
                })?;

            let data = InsertableRoleMemberTranslateList {
                role_member_id: new_role_member_id,
                lang_id: data.lang_id,
                name: data.name.to_string(),
            };

            diesel::insert_into(role_member_translate_list::role_member_translate_list)
                .values(&data)
                .returning(role_member_translate_list::role_member_id)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert role_member_translate_list: {:?} ", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
