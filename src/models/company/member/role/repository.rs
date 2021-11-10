use crate::errors::ServiceResult;
use crate::models::company::member::role::model::{
    RoleMember,
    RoleMemberTranslateList,
    RoleMemberAndRelatedData,
};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::role_member_translate_list::dsl::*;
use diesel::prelude::*;

impl RoleMember {
    // /// Get roles IDs have target access level
    // pub(crate) fn get_roles_for_type_access(
    //     target_type_access: &i32,
    //     conn: &PgConnection,
    // ) -> Vec<i32> {
    //     use crate::schema::role_access::dsl::*;
    //
    //     let role = role_access
    //         .filter(type_access_id.le(target_type_access))
    //         .select(role_id)
    //         .load::<i32>(conn);
    //
    //     // if not found data for set lang
    //     match role {
    //         Ok(rl) => rl,
    //         Err(err) => {
    //             debug!("Not found suitable roles: {:?}", err);
    //
    //             Vec::new()
    //         },
    //     }
    // }
}

impl RoleMemberTranslateList {
    // /// Get role data with translate by role id
    // pub(crate) fn get_role_by_id(
    //     target_role_id: &i32,
    //     set_lang_id: &i32,
    //     conn: &PgConnection,
    // ) -> ServiceResult<RoleMemberTranslateList> {
    //     let role = role_member_translate_list
    //         .filter(role_member_id.eq(target_role_id)
    //         .and(lang_id.eq(set_lang_id)))
    //         .first::<RoleMemberTranslateList>(conn);
    //
    //     // if not found data for set lang
    //     match role {
    //         Ok(rn) => Ok(rn),
    //         Err(err) => {
    //             debug!("Not found set lang for role: {:?}", err);
    //             Ok(role_member_translate_list
    //                 .filter(role_member_id.eq(target_role_id))
    //                 .first::<RoleMemberTranslateList>(conn)?)
    //         },
    //     }
    // }

    /// Get roles data with translate by roles IDs
    pub(crate) fn get_roles_by_ids(
        target_roles_ids: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<RoleMemberTranslateList>> {
        let roles = role_member_translate_list
            .filter(role_member_id.eq_any(target_roles_ids)
            .and(lang_id.eq(set_lang_id)))
            .load::<RoleMemberTranslateList>(conn);

        // if not found data for set lang
        match roles {
            Ok(rns) => Ok(rns),
            Err(err) => {
                debug!("Not found set lang for roles: {:?}", err);
                Ok(role_member_translate_list
                    .filter(role_member_id.eq_any(target_roles_ids))
                    .load::<RoleMemberTranslateList>(conn)?)
            },
        }
    }
}

impl RoleMemberAndRelatedData {
    // /// Get role by id for set lang
    // pub fn get_role_by_id(
    //     target_role_id: &i32,
    //     set_lang_id: &i32,
    //     conn: &PgConnection,
    // ) -> ServiceResult<RoleMemberAndRelatedData> {
    //     let role = RoleMemberTranslateList::get_role_by_id(
    //         target_role_id,
    //         set_lang_id,
    //         conn
    //     )?;
    //
    //     let access = TypeAccessTranslateList::get_by_role_id(
    //         target_role_id,
    //         set_lang_id,
    //         conn
    //     )?;
    //
    //     // if found data return RoleMemberAndRelatedData
    //     Ok(RoleMemberAndRelatedData {
    //         role,
    //         access,
    //     })
    // }

    /// Get roles by IDs for set lang
    pub(crate) fn get_roles_by_ids(
        target_roles_ids: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
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
        conn: &PgConnection,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        use crate::schema::role_access::dsl as role_access;

        let target_types_access_ids = role_access::role_access
            .filter(role_access::role_id.eq(target_role_id))
            .select(role_access::type_access_id)
            .load::<i32>(conn)?;

        TypeAccessTranslateList::get_types_access_by_ids(
            &target_types_access_ids,
            set_lang_id,
            conn
        )
    }
}
