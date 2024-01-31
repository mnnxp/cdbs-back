use crate::schema::*;
use crate::models::user::model::UserQuery;
use crate::models::company::{
    model::Company,
    member::role::model::{RoleMember, RoleMemberAndRelatedData},
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

/// Company (community) member data
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyMemberAndRelatedData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// User's role in the company (access rights are granted based on the role)
    pub(crate) role: RoleMemberAndRelatedData,
    /// Activity flag of the company member
    pub(crate) is_enabled: bool,
    /// Date the user was added to the company
    pub(crate) created_at: NaiveDateTime,
    /// Date the user's role or activity was changed
    pub(crate) updated_at: NaiveDateTime,
}

/// Abbreviated data about the company member
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct SlimCompanyMember {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// Identifier of the user's role in the company
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

/// Data for adding or changing the role of a company member
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyMemberData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
    /// Identifier of the user's role in the company
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

/// Deactivation (deletion of role/access) of a company member
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyMemberData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// User UUID
    pub(crate) user_uuid: Uuid,
}
