use crate::schema::*;
use crate::models::user::model::User;
use crate::models::company::model::Company;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

// Favorites company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_company, uuid_company)]
#[belongs_to(Company, foreign_key = "uuid_company")]
#[belongs_to(User, foreign_key = "uuid_user")]
#[table_name = "company_fav"]
pub struct CompanyFav {
    pub uuid_company: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

#[Object]
impl CompanyFav {
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
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
    pub uuid_company: Uuid,
    pub uuid_user: Uuid,
    // pub is_enabled: bool,
    // pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_fav"]
pub struct InsertableCompanyFav {
    pub uuid_company: Uuid,
    pub uuid_user: Uuid,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
}

impl From<IptCompanyFavData> for InsertableCompanyFav {
    fn from(ipt_data: IptCompanyFavData) -> Self {
        let IptCompanyFavData {
            uuid_company,
            uuid_user,
            // is_enabled,
            // created_at,
            ..
        } = ipt_data;

        Self {
            uuid_company: Uuid::parse_str(&uuid_company.to_string()).unwrap(),
            uuid_user: Uuid::parse_str(&uuid_user.to_string()).unwrap(),
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
