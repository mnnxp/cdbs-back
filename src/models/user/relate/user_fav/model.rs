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
pub(crate) struct UserFav {
    pub(crate) user_favorite_uuid: Uuid,
    pub(crate) user_follower_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserFavData {
    pub(crate) user_favorite_uuid: Uuid,
    pub(crate) user_follower_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "user_fav"]
pub(crate) struct InsertableUserFav {
    pub(crate) user_favorite_uuid: Uuid,
    pub(crate) user_follower_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

impl From<IptUserFavData> for InsertableUserFav {
    fn from(ipt_data: IptUserFavData) -> Self {
        let IptUserFavData {
            user_favorite_uuid,
            user_follower_uuid,
        } = ipt_data;

        Self {
            user_favorite_uuid,
            user_follower_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
