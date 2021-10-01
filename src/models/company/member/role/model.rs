use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
use uuid::Uuid;

// RoleMember models
#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "role_member_list"]
pub struct RoleMember {
    pub id: i32,
    pub company_uuid: Uuid,
}

#[Object]
impl RoleMember {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "role_member_list"]
pub struct InsertableRoleMember {
    pub id: i32,
    pub company_uuid: Uuid,
}

// RoleMember translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Default, Clone, Debug)]
#[primary_key(role_member_id, lang_id)]
#[belongs_to(RoleMember, foreign_key = "role_member_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "role_member_translate_list"]
pub struct RoleMemberTranslateList {
    pub role_member_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[Object]
impl RoleMemberTranslateList {
    async fn role_member_id(&self) -> &i32 {
        &self.role_member_id
    }
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
pub struct RoleMemberAndRelatedData {
    // pub company_uuid: Uuid,
    // show role data with translation
    pub role: RoleMemberTranslateList,
    // show access types for this role
    pub access: Vec<TypeAccessTranslateList>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRoleMemberData {
    pub company_uuid: Uuid,
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "role_member_translate_list"]
pub struct InsertableRoleMemberTranslateList {
    pub role_member_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelRoleMemberData {
    pub company_uuid: Uuid,
    pub role_id: i32,
}
