use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(component_uuid, user_uuid)]
#[table_name = "user_access_to_component"]
pub struct UserAccessComponent {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct UserAccessComponentAndRelatedData {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access: TypeAccessTranslateList,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_access_to_component"]
pub struct InsertableUserAccessComponent {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptUserAccessComponentData {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
}

impl From<&IptUserAccessComponentData> for InsertableUserAccessComponent {
    fn from(data_component: &IptUserAccessComponentData) -> Self {
        let IptUserAccessComponentData {
            component_uuid,
            user_uuid,
            type_access_id,
            ..
        } = data_component;

        Self {
            component_uuid: *component_uuid,
            user_uuid: *user_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Debug, Deserialize, InputObject)]
pub struct DelUserAccessComponentData {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
}
