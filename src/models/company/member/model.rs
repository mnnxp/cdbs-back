use crate::schema::*;
use crate::models::user::model::UserQuery;
use crate::models::company::model::Company;
use crate::models::company::member::role::model::{
    RoleMember, RoleMemberAndRelatedData
};
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(company_uuid, user_uuid))]
#[diesel(belongs_to(Company, foreign_key = company_uuid))]
#[diesel(belongs_to(UserQuery, foreign_key = user_uuid))]
#[diesel(belongs_to(RoleMember, foreign_key = role_id))]
#[diesel(table_name = company_member_list)]
pub(crate) struct CompanyMember {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) role_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyMemberAndRelatedData {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) role: RoleMemberAndRelatedData,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct SlimCompanyMember {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) role_id: i32,
}

impl From<CompanyMember> for SlimCompanyMember {
    fn from(data: CompanyMember) -> Self {
        let CompanyMember {
            company_uuid,
            user_uuid,
            role_id,
            ..
        } = data;

        Self {
            company_uuid,
            user_uuid,
            role_id,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_member_list)]
pub(crate) struct InsertableCompanyMember {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) role_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyMemberData {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) role_id: i32,
}

impl From<&IptCompanyMemberData> for InsertableCompanyMember {
    fn from(ipt_data: &IptCompanyMemberData) -> Self {
        let IptCompanyMemberData {
            company_uuid,
            user_uuid,
            role_id,
            ..
        } = ipt_data;

        Self {
            company_uuid: *company_uuid,
            user_uuid: *user_uuid,
            role_id: *role_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyMemberData {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
}
