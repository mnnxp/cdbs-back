use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::user::model::UserQuery;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites user models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(company_uuid, user_uuid)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[belongs_to(UserQuery, foreign_key = "user_uuid")]
#[table_name = "company_fav"]
pub struct CompanyFav {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl CompanyFav {
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyFavData {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "company_fav"]
pub struct InsertableCompanyFav {
    pub company_uuid: Uuid,
    pub user_uuid: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
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
