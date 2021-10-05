use crate::errors::ServiceResult;
// use crate::models::company::model::Company;
use crate::models::company::member::model::{
    CompanyMember,
    CompanyMemberAndRelatedData,
};
use crate::models::company::member::role::model::RoleMemberAndRelatedData;
use diesel::prelude::*;
use uuid::Uuid;


impl CompanyMember {
    // /// Gets company member without related data by Company
    // pub(crate) fn get_by_company(
    //     company: &Company,
    //     conn: &PgConnection,
    // ) -> ServiceResult<Vec<CompanyMember>> {
    //     // collect data for members the company
    //     Ok(CompanyMember::belonging_to(company)
    //         .load::<CompanyMember>(conn)?
    //     )
    // }

    /// Gets company member without related data by company uuid
    pub(crate) fn get_by_company_uuid(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyMember>> {
        use crate::schema::company_member_list::dsl::*;

        // collect data for members the company
        Ok(company_member_list
            .filter(company_uuid.eq(target_company_uuid))
            .load::<CompanyMember>(conn)?
        )
    }

    // /// Gets company member without related data by users uuids
    // pub(crate) fn get_by_users_uuids(
    //     target_company_uuid: &Uuid,
    //     users_uuids: &[Uuid],
    //     conn: &PgConnection,
    // ) -> ServiceResult<Vec<CompanyMember>> {
    //     use crate::schema::company_member_list::dsl::*;
    //
    //     // collect data for members the company
    //     Ok(company_member_list
    //         .filter(company_uuid.eq(target_company_uuid)
    //         .and(user_uuid.eq_any(users_uuids)))
    //         .load::<CompanyMember>(conn)?
    //     )
    // }
}

impl CompanyMemberAndRelatedData {
    /// Gets company members by company uuid
    /// and role data with translation for a given language
    pub(crate) fn get_list_members_by_company_uuid(
        company_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        let company_members = &CompanyMember::get_by_company_uuid(
            company_uuid,
            conn
        ).unwrap();

        CompanyMemberAndRelatedData::get_related_data_for_members(
            company_members,
            set_lang_id,
            conn
        )
    }

    // /// Gets company members by members uuids
    // /// and role data with translation for a given language
    // pub(crate) fn get_company_members_by_uuid(
    //     target_company_uuid: &Uuid,
    //     members_uuids: &[Uuid],
    //     set_lang_id: &i32,
    //     conn: &PgConnection,
    // ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
    //     let company_members = &CompanyMember::_get_by_users_uuids(
    //         target_company_uuid,
    //         members_uuids,
    //         conn
    //     )?;
    //
    //     CompanyMemberAndRelatedData::get_related_data_for_members(
    //         company_members,
    //         set_lang_id,
    //         conn
    //     )
    // }

    /// Gets company members with related data by company members
    /// and role data with translation for a given language
    pub(crate) fn get_related_data_for_members(
        company_members: &[CompanyMember],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        let mut member_roles_ids: Vec<i32> = Vec::new();
        // selecting member roles for gets translate data
        for member in company_members.iter() {
            member_roles_ids.push(member.role_id);
        }

        // get member types for company members
        let member_roles = RoleMemberAndRelatedData::get_roles_by_ids(
            &member_roles_ids,
            set_lang_id,
            conn
        )?;

        // debug!("Company member member_role_list_id: {:#?}", member_role_list_id);

        let mut company_member_with_role: Vec<CompanyMemberAndRelatedData> = Vec::new();
        for member in company_members {
            let mut member_role_data = &RoleMemberAndRelatedData::default();

            // find member role with translate for target member
            for member_role in &member_roles {
                if member.role_id == member_role.role.role_member_id {
                    member_role_data = member_role;
                    break;
                }
            }

            company_member_with_role.push(CompanyMemberAndRelatedData{
                company_uuid: member.company_uuid.to_owned(),
                user_uuid: member.user_uuid.to_owned(),
                role: member_role_data.to_owned(),
                is_enabled: member.is_enabled.to_owned(),
                created_at: member.created_at.to_owned(),
                updated_at: member.updated_at.to_owned(),
            })
        }

        Ok(company_member_with_role)
    }
}
