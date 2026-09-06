use crate::schema::*;
use chrono::*;
use uuid::Uuid;

// Favorites component models
#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
#[diesel(table_name = component_fav)]
pub(crate) struct ComponentFav {
    pub(crate) component_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
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
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
