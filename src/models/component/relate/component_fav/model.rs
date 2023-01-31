use crate::schema::*;
use crate::models::user::model::User;
use crate::models::component::model::Component;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(component_uuid, component_uuid))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(belongs_to(User, foreign_key = user_uuid))]
#[diesel(table_name = component_fav)]
pub(crate) struct ComponentFav {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentFavData {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    // pub(crate) is_enabled: bool,
    // pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_fav)]
pub(crate) struct InsertableComponentFav {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

impl From<IptComponentFavData> for InsertableComponentFav {
    fn from(ipt_data: IptComponentFavData) -> Self {
        let IptComponentFavData {
            component_uuid,
            user_uuid,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            component_uuid,
            user_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
