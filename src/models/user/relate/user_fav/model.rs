use crate::schema::*;
use crate::models::user::model::UserQuery;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites user models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_user_favorite, uuid_user_follower)]
#[belongs_to(UserQuery, foreign_key = "uuid_user_favorite", "uuid_user_follower")]
#[table_name = "user_fav"]
pub struct UserFav {
    pub uuid_user_favorite: Uuid,
    pub uuid_user_follower: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl UserFav {
    async fn uuid_user_favorite(&self) -> ID {
        self.uuid_user_favorite.into()
    }
    async fn uuid_user_follower(&self) -> ID {
        self.uuid_user_follower.into()
    }
    async fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserFavData {
    pub uuid_user_favorite: Uuid,
    pub uuid_user_follower: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_fav"]
pub struct InsertableUserFav {
    pub uuid_user_favorite: Uuid,
    pub uuid_user_follower: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<IptUserFavData> for InsertableUserFav {
    fn from(ipt_data: IptUserFavData) -> Self {
        let IptUserFavData {
            uuid_user_favorite,
            uuid_user_follower,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            uuid_user_favorite: Uuid::parse_str(&uuid_user_favorite.to_string()).unwrap(),
            uuid_user_follower: Uuid::parse_str(&uuid_user_follower.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
