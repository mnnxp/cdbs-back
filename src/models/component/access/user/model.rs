use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(component_uuid, user_uuid))]
#[diesel(table_name = user_access_to_component)]
pub(crate) struct UserAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct UserAccessComponentAndRelatedData {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access: TypeAccessTranslateList,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_access_to_component)]
pub(crate) struct InsertableUserAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptUserAccessComponentData {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
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
pub(crate) struct DelUserAccessComponentData {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
}
