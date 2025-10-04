use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites company models
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = company_fav)]
pub(crate) struct CompanyFav {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

/// Data for adding a company to the user's favorites
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IptCompanyFavData {
    /// UUID of the company to be added
    pub(crate) company_uuid: Uuid,
    /// UUID of the user to whose favorites the company is added
    pub(crate) user_uuid: Uuid,
    // pub(crate) is_enabled: bool,
    // pub(crate) created_at: NaiveDateTime,
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
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            company_uuid,
            user_uuid,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
