use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use async_graphql::*;
use uuid::Uuid;

/// Идентификатор роли участников для компании
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = role_member_list)]
pub(crate) struct RoleMember {
    /// Идентификатор роли
    pub(crate) id: i32,
    /// Идентификатор компании
    pub(crate) company_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = role_member_list)]
pub(crate) struct InsertableRoleMember {
    pub(crate) id: i32,
    pub(crate) company_uuid: Uuid,
}

/// Наименование роли участников компании с локализацией
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Default, Clone, Debug)]
#[diesel(primary_key(role_member_id, lang_id))]
#[diesel(belongs_to(RoleMember, foreign_key = role_member_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = role_member_translate_list)]
pub(crate) struct RoleMemberTranslateList {
    /// Идентификатор роли
    pub(crate) role_member_id: i32,
    /// Идентификатор языка
    pub(crate) lang_id: i32,
    /// Наименование роли
    pub(crate) name: String,
}

/// Уровень доступа для роли с локализацией
#[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
pub(crate) struct RoleMemberAndRelatedData {
    /// Данные о роли компании
    pub(crate) role: RoleMemberTranslateList,
    /// Данные об уровне доступа для роли
    pub(crate) access: Vec<TypeAccessTranslateList>,
}

/// Данные для добавления новой роли компании/сообщества
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRoleMemberData {
    /// Идентификатор компании
    pub(crate) company_uuid: Uuid,
    /// Идентификатор языка (на котором указано наименование)
    pub(crate) lang_id: i32,
    /// Наименование роли в рамках компании
    pub(crate) name: String,
}

/// Данные для обновления роли компании/сообщества
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateNameRoleData {
    /// Идентификатор компании
    pub(crate) company_uuid: Uuid,
    /// Идентификатор роли компании
    pub(crate) role_id: i32,
    /// Идентификатор языка (на котором указано наименование)
    pub(crate) lang_id: i32,
    /// Наименование роли в рамках компании
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = role_member_translate_list)]
pub(crate) struct InsertableRoleMemberTranslateList {
    pub(crate) role_member_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

/// Удаление роли компании
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelRoleMemberData {
    /// Идентификатор компании
    pub(crate) company_uuid: Uuid,
    /// Идентификатор роли
    pub(crate) role_id: i32,
}
