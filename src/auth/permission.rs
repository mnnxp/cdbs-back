use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

/// Permission representation for RBAC (Manage, Write, Read)
/// This is a GraphQL wrapper over TypeAccessTranslateList with renamed levels
#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct PermissionTranslateList {
    pub(crate) type_access_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

impl From<TypeAccessTranslateList> for PermissionTranslateList {
    fn from(access: TypeAccessTranslateList) -> Self {
        Self {
            type_access_id: access.type_access_id,
            lang_id: access.lang_id,
            name: Self::fix_name_to_level(&access),
        }
    }
}

impl From<&TypeAccessTranslateList> for PermissionTranslateList {
    fn from(access: &TypeAccessTranslateList) -> Self {
        Self {
            type_access_id: access.type_access_id,
            lang_id: access.lang_id,
            name: Self::fix_name_to_level(access),
        }
    }
}

impl PermissionTranslateList {
    fn fix_name_to_level(access: &TypeAccessTranslateList) -> String {
        match (access.type_access_id, access.lang_id) {
            (1, 1) => "Manage".to_string(),
            (1, 2) => "Управление".to_string(),
            (1, 3) => "管理".to_string(),
            (2, 1) => "Write".to_string(),
            (2, 2) => "Запись".to_string(),
            (2, 3) => "写".to_string(),
            (3, 1) => "Read".to_string(),
            (3, 2) => "Чтение".to_string(),
            (3, 3) => "读".to_string(),
            _ => access.name.clone(),
        }
    }
}
