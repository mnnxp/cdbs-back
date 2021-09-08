use crate::schema::*;
use crate::models::user::model::User;
use crate::models::standard::model::Standard;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_standard, uuid_standard)]
#[belongs_to(Standard, foreign_key = "uuid_standard")]
#[belongs_to(User, foreign_key = "uuid_user")]
#[table_name = "standard_fav"]
pub struct StandardFav {
    pub uuid_standard: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl StandardFav {
    async fn uuid_standard(&self) -> ID {
        self.uuid_standard.into()
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
pub struct IptStandardFavData {
    pub uuid_standard: Uuid,
    pub uuid_user: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_fav"]
pub struct InsertableStandardFav {
    pub uuid_standard: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<IptStandardFavData> for InsertableStandardFav {
    fn from(ipt_data: IptStandardFavData) -> Self {
        let IptStandardFavData {
            uuid_standard,
            uuid_user,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            uuid_standard: Uuid::parse_str(&uuid_standard.to_string()).unwrap(),
            uuid_user: Uuid::parse_str(&uuid_user.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
