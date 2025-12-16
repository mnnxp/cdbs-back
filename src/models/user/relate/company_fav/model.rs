use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IptCompanyFavData {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_fav)]
pub(crate) struct InsertableCompanyFav {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

impl From<IptCompanyFavData> for InsertableCompanyFav {
    fn from(ipt_data: IptCompanyFavData) -> Self {
        let IptCompanyFavData {
            company_uuid,
            user_uuid,
        } = ipt_data;

        Self {
            company_uuid,
            user_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
