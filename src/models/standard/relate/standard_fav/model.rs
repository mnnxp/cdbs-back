use crate::schema::*;
use crate::models::user::model::User;
use crate::models::standard::model::Standard;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(standard_uuid, standard_uuid)]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "standard_fav"]
pub struct StandardFav {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardFavData {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_fav"]
pub struct InsertableStandardFav {
    pub standard_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<IptStandardFavData> for InsertableStandardFav {
    fn from(ipt_data: IptStandardFavData) -> Self {
        let IptStandardFavData {
            standard_uuid,
            user_uuid,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            standard_uuid: Uuid::parse_str(&standard_uuid.to_string()).unwrap(),
            user_uuid: Uuid::parse_str(&user_uuid.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
