use crate::schema::*;
use crate::models::user::model::User;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, component_uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "component_fav"]
pub struct ComponentFav {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl ComponentFav {
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentFavData {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "component_fav"]
pub struct InsertableComponentFav {
    pub component_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
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
            component_uuid: Uuid::parse_str(&component_uuid.to_string()).unwrap(),
            user_uuid: Uuid::parse_str(&user_uuid.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
