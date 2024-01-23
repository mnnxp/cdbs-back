use crate::schema::*;
use crate::models::user::model::User;
use crate::models::company::model::Company;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(company_uuid, company_uuid))]
#[diesel(belongs_to(Company, foreign_key = company_uuid))]
#[diesel(belongs_to(User, foreign_key = user_uuid))]
#[diesel(table_name = company_fav)]
pub(crate) struct CompanyFav {
    pub(crate) company_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
}

/// Данные для добавления компании в избранное пользователя
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IptCompanyFavData {
    /// UUID компании которую требуется добавить
    pub(crate) company_uuid: Uuid,
    /// UUID пользователя в избранное которого добавляется компания
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
