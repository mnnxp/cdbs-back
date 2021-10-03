use crate::schema::*;
use crate::models::user::model::UserQuery;
use crate::models::company::model::Company;
use crate::models::company::member::role::model::{
    RoleMember, RoleMemberAndRelatedData
};
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(company_uuid, user_uuid)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[belongs_to(UserQuery, foreign_key = "user_uuid")]
#[belongs_to(RoleMember, foreign_key = "role_id")]
#[table_name = "company_member_list"]
pub struct CompanyMember {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl CompanyMember {
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn role_id(&self) -> &i32 {
        &self.role_id
    }
    async fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CompanyMemberAndRelatedData {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role: RoleMemberAndRelatedData,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct SlimCompanyMember {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role_id: i32,
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
#[table_name = "company_member_list"]
pub struct InsertableCompanyMember {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyMemberData {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role_id: i32,
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
pub struct DelCompanyMemberData {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
}
