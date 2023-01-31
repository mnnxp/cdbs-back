use crate::schema::*;
use crate::models::user::model::User;
use crate::models::standard::model::Standard;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(standard_uuid, standard_uuid))]
#[diesel(belongs_to(Standard, foreign_key = standard_uuid))]
#[diesel(belongs_to(User, foreign_key = user_uuid))]
#[diesel(table_name = standard_fav)]
pub(crate) struct StandardFav {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardFavData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    // pub(crate) is_enabled: bool,
    // pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_fav)]
pub(crate) struct InsertableStandardFav {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
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
            standard_uuid,
            user_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
