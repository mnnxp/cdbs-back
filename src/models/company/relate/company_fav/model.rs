use crate::schema::*;
use crate::models::user::model::User;
use crate::models::company::model::Company;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(company_uuid, company_uuid)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "company_fav"]
pub(crate) struct CompanyFav {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyFavData {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    // pub(crate) is_enabled: bool,
    // pub(crate) created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_fav"]
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
