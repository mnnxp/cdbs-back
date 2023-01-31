use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::member::role::model::{
    RoleMemberTranslateList, RoleMemberAndRelatedData,
};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::role_member_translate_list::dsl::*;
use diesel::prelude::*;

impl RoleMemberTranslateList {
    /// Get role data with translate by role id
    pub(crate) fn get_by_id(
        target_role_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<RoleMemberTranslateList> {
        let role = role_member_translate_list
            .filter(role_member_id.eq(target_role_id)
            .and(lang_id.eq(set_lang_id)))
            .limit(1)
            .load::<RoleMemberTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get role member: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match role.first() {
            Some(x) => Ok(x.clone()),
            None => {
                debug!("Not found set lang for role");
                role_member_translate_list
                    .filter(role_member_id.eq(target_role_id))
                    .first::<RoleMemberTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get role: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }

    /// Get roles data with translate by roles IDs
    pub(crate) fn get_roles_by_ids(
        target_roles_ids: &[i32],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<RoleMemberTranslateList>> {
        let roles = role_member_translate_list
            .filter(role_member_id.eq_any(target_roles_ids)
            .and(lang_id.eq(set_lang_id)))
            .load::<RoleMemberTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed get role access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // if not found data for set lang
        match roles.is_empty() {
            true => {
                debug!("Not found set lang for roles");
                role_member_translate_list
                    .filter(role_member_id.eq_any(target_roles_ids))
                    .load::<RoleMemberTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed get role: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
            false => Ok(roles),
        }
    }
}

impl RoleMemberAndRelatedData {
    /// Get role by id for set lang
    pub(crate) fn get_by_id(
        target_role_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<RoleMemberAndRelatedData> {
        let role = RoleMemberTranslateList::get_by_id(
            target_role_id,
            set_lang_id,
            conn
        )?;

        let access = TypeAccessTranslateList::get_by_role_id(
            target_role_id,
            set_lang_id,
            conn
        )?;

        // if found data return RoleMemberAndRelatedData
        Ok(RoleMemberAndRelatedData {
            role,
            access,
        })
    }

    /// Get roles by IDs for set lang
    pub(crate) fn get_roles_by_ids(
        target_roles_ids: &[i32],
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<RoleMemberAndRelatedData>> {
        let roles = RoleMemberTranslateList::get_roles_by_ids(
            target_roles_ids,
            set_lang_id,
            conn
        )?;

        let mut res: Vec<RoleMemberAndRelatedData> = Vec::new();
        for role in roles {
            // get access types for the role
            let access = TypeAccessTranslateList::get_by_role_id(
                &role.role_member_id,
                set_lang_id,
                conn
            )?;

            res.push(RoleMemberAndRelatedData {
                role,
                access,
            })
        }

        Ok(res)
    }
}

impl TypeAccessTranslateList {
    /// Get access type for target role id
    pub(crate) fn get_by_role_id(
        target_role_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        use crate::schema::role_access::dsl as role_access;

        let target_types_access_ids = role_access::role_access
            .filter(role_access::role_id.eq(target_role_id))
            .select(role_access::type_access_id)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get role access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        TypeAccessTranslateList::get_types_access_by_ids(
            &target_types_access_ids,
            set_lang_id,
            conn
        )
    }
}
