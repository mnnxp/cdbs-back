use crate::schema::*;
use crate::models::user::model::User;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_component, uuid_component)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(User, foreign_key = "uuid_user")]
#[table_name = "component_fav"]
pub struct ComponentFav {
    pub uuid_component: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl ComponentFav {
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
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
    pub uuid_component: Uuid,
    pub uuid_user: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "component_fav"]
pub struct InsertableComponentFav {
    pub uuid_component: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<IptComponentFavData> for InsertableComponentFav {
    fn from(ipt_data: IptComponentFavData) -> Self {
        let IptComponentFavData {
            uuid_component,
            uuid_user,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            uuid_user: Uuid::parse_str(&uuid_user.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
