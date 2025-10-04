use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

/// Participant role identifier for the company
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = role_member_list)]
pub(crate) struct RoleMember {
    /// Role ID
    pub(crate) id: i32,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = role_member_list)]
pub(crate) struct InsertableRoleMember {
    pub(crate) id: i32,
    pub(crate) company_uuid: Uuid,
}

/// Role name of company members with localization
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Default, Clone, Debug)]
#[diesel(table_name = role_member_translate_list)]
pub(crate) struct RoleMemberTranslateList {
    /// Role identifier
    pub(crate) role_member_id: i32,
    /// Language identifier
    pub(crate) lang_id: i32,
    /// Role name
    pub(crate) name: String,
}

/// Access level for role with localization
#[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
pub(crate) struct RoleMemberAndRelatedData {
    /// Company role data
    pub(crate) role: RoleMemberTranslateList,
    /// Access level data for the role
    pub(crate) access: Vec<TypeAccessTranslateList>,
}

/// Data for adding a new company/community role
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRoleMemberData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Language identifier (in which the name is specified)
    pub(crate) lang_id: i32,
    /// Name of the role within the company
    pub(crate) name: String,
}

/// Data for updating the company/community role
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateNameRoleData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Company role identifier
    pub(crate) role_id: i32,
    /// Language identifier (in which the name is specified)
    pub(crate) lang_id: i32,
    /// Name of the role within the company
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = role_member_translate_list)]
pub(crate) struct InsertableRoleMemberTranslateList {
    pub(crate) role_member_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

/// Deleting a company role
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelRoleMemberData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Role ID
    pub(crate) role_id: i32,
}
