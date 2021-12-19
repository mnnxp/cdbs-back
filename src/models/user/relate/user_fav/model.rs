use crate::schema::*;
use crate::models::user::model::UserQuery;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites user models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(user_favorite_uuid, user_follower_uuid)]
#[belongs_to(UserQuery, foreign_key = "user_favorite_uuid", "user_follower_uuid")]
#[table_name = "user_fav"]
pub struct UserFav {
    pub user_favorite_uuid: Uuid,
    pub user_follower_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserFavData {
    pub user_favorite_uuid: Uuid,
    pub user_follower_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "user_fav"]
pub struct InsertableUserFav {
    pub user_favorite_uuid: Uuid,
    pub user_follower_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<&IptUserFavData> for InsertableUserFav {
    fn from(ipt_data: &IptUserFavData) -> Self {
        let IptUserFavData {
            user_favorite_uuid,
            user_follower_uuid,
            ..
        } = ipt_data;

        Self {
            user_favorite_uuid: *user_favorite_uuid,
            user_follower_uuid: *user_follower_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
