use crate::errors::{ServiceResult, ServiceError};
// use crate::models::company::model::Company;
use crate::models::company::member::model::{
    CompanyMember, CompanyMemberAndRelatedData,
};
use crate::models::company::member::role::model::RoleMemberAndRelatedData;
use crate::schema::company_member_list::dsl as company_member_list;
use diesel::prelude::*;
use uuid::Uuid;


impl CompanyMember {
    /// Gets company member without related data by company uuid
    pub(crate) fn get_by_company_uuid(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyMember>> {
        // collect data for members the company
        company_member_list::company_member_list
            .filter(company_member_list::company_uuid.eq(target_company_uuid))
            .load::<CompanyMember>(conn)
            .map_err(|err| {
                debug!("Failed get company_member_list: {:?} ", err);
                ServiceError::InternalServerError
            })
    }
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
        )?;

        CompanyMemberAndRelatedData::get_related_data_for_members(
            company_members,
            set_lang_id,
            conn
        )
    }

    /// Gets company members with related data by company members
    /// and role data with translation for a given language
    pub(crate) fn get_related_data_for_members(
        company_members: &[CompanyMember],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        let mut company_member_with_role: Vec<CompanyMemberAndRelatedData> = Vec::new();

        for member in company_members {
            // get member types for company members
            let member_role = RoleMemberAndRelatedData::get_by_id(
                &member.role_id,
                set_lang_id,
                conn
            )?;

            company_member_with_role.push(CompanyMemberAndRelatedData{
                company_uuid: member.company_uuid,
                user_uuid: member.user_uuid,
                role: member_role.clone(),
                is_enabled: member.is_enabled,
                created_at: member.created_at,
                updated_at: member.updated_at,
            })
        }

        Ok(company_member_with_role)
    }
}
