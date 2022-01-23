use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use async_graphql::*;
use uuid::Uuid;

// RoleMember models
#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, SimpleObject, Debug)]
#[primary_key(id)]
#[table_name = "role_member_list"]
pub(crate) struct RoleMember {
    pub(crate) id: i32,
    pub(crate) company_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "role_member_list"]
pub(crate) struct InsertableRoleMember {
    pub(crate) id: i32,
    pub(crate) company_uuid: Uuid,
}

// RoleMember translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Default, Clone, Debug)]
#[primary_key(role_member_id, lang_id)]
#[belongs_to(RoleMember, foreign_key = "role_member_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "role_member_translate_list"]
pub(crate) struct RoleMemberTranslateList {
    pub(crate) role_member_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
pub(crate) struct RoleMemberAndRelatedData {
    // pub(crate) company_uuid: Uuid,
    // show role data with translation
    pub(crate) role: RoleMemberTranslateList,
    // show access types for this role
    pub(crate) access: Vec<TypeAccessTranslateList>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRoleMemberData {
    pub(crate) company_uuid: Uuid,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateNameRoleData {
    pub(crate) company_uuid: Uuid,
    pub(crate) role_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "role_member_translate_list"]
pub(crate) struct InsertableRoleMemberTranslateList {
    pub(crate) role_member_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelRoleMemberData {
    pub(crate) company_uuid: Uuid,
    pub(crate) role_id: i32,
}
