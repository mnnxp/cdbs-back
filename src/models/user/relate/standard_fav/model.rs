use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IptStandardFavData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
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
        } = ipt_data;

        Self {
            standard_uuid,
            user_uuid,
            is_enabled: true,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
